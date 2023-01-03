// HANDICAP: Use parallelization (thread pool seems easy and most applicable?)
// It would be dumb in this case to use a "sliding window" approach -- at least not a trivial one
// Need to keep bookmark on done windows (e.g., idx = 1...4), this way threads can perform 
// non-ordered processing. 

// Questions seems to imply single-thread processing (*first* marker), but... suffices to use a min-
// heuristic when threads pick their tasks to instantly return when all prev per-4-idxs are NOT in 
// available tasks! e.g., when one start marker is found, shrink the job pool to < that start marker.

use super::AOCSolutions; 
use std::collections::{VecDeque, HashSet};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;  

pub struct Day6; 
struct PooledDecoder {
    data: Vec<u8>, 
    available_jobs: VecDeque<usize>, 
    config: usize, 
    decoders: Vec<Decoder>, 
    job_tx: Option<mpsc::Sender<(usize, Vec<u8>)>>,  // Send u8 subslice to individual decoder threads
    bound_rx: mpsc::Receiver<usize>, // Receive results from threads to alter u8 slice range
}

impl PooledDecoder {
    pub fn new(data: Vec<u8>, config: usize, thread_count: usize) -> PooledDecoder {
        assert!(thread_count > 0); 
        assert!(config > 0); 
        assert!(data.len() >= config);
        
        let idx_bgn_candidates: VecDeque<usize> = (0..=(data.len() - config)).collect(); // Sorted

        let (job_tx, job_rx) = mpsc::channel::<(usize, Vec<u8>)>(); 
        let (bound_tx, bound_rx) = mpsc::channel::<usize>(); 
        let job_rx = Arc::new(Mutex::new(job_rx)); 

        let mut decoders = Vec::with_capacity(thread_count); 
        for id in 0..thread_count {
            let bound_tx = bound_tx.clone(); 
            decoders.push(Decoder::new(id, bound_tx, Arc::clone(&job_rx))); 
        }

        return PooledDecoder{
            data, 
            available_jobs: idx_bgn_candidates, 
            config, 
            decoders, 
            job_tx: Some(job_tx),  
            bound_rx, 
        }; 
    }

    pub fn execute(&mut self) -> Option<usize> {
        let mut best: Option<usize> = None;
        // Send jobs
        while !self.available_jobs.is_empty() { 
            let job = self.available_jobs.pop_front().unwrap();
            self.job_tx.as_ref().unwrap().send((job, self.data[job..(job + self.config)].to_vec()))
                .unwrap(); 
        }

        // Receive results
        loop {
            match self.bound_rx.recv_timeout(Duration::from_secs(1)) {
                Ok(new_bound) => {
                    let new_bound = new_bound + self.config; // align to rest of substring
                    match best {
                        None => best = Some(new_bound), 
                        Some(v) => if v > new_bound { best = Some(new_bound) }, 
                    }
                }, 
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    panic!("[day6::PooledDecoder::execute] Unexpected `mpsc` receiver closure from `Decoder` client")
                }, 
                _ => break, 
            }
        }
        return best; 
    }
}

impl Drop for PooledDecoder {
    fn drop(&mut self) {
        drop(self.job_tx.take()); 

        for decoder in &mut self.decoders {
            if let Some(handle) = decoder.handle.take() {
                handle.join().unwrap(); 
                eprintln!("[day6::PooledDecoder::drop] Decoder disconnected"); 
            }
        }
    }
}
struct Decoder {
    id: usize, 
    handle: Option<thread::JoinHandle<()>>, 
}

impl Decoder {
    fn new (id: usize, bound_tx: mpsc::Sender<usize>, job_rx: Arc<Mutex<mpsc::Receiver<(usize, Vec<u8>)>>>) -> Decoder {
        let handle = thread::spawn(move || 
            loop {
                match job_rx.lock().unwrap().recv() { 
                    Ok((idx, u8_subvec)) => {
                        if Day6::is_packet_start_marker(&u8_subvec) {
                            bound_tx.send(idx).unwrap();  
                        }
                    }, 
                    Err(_) => break, // PooledDecoder deconnection  
                }
            }
        ); 
        
        return Decoder{id, handle: Some(handle)}; 
    }
}

impl AOCSolutions for Day6 {
    fn get_star_1(input: &str) -> Result<i64, ()> {
        match Day6::pooled_decode(input.as_bytes(), 4, 8) {
            Some(v) => return Ok(v.try_into().unwrap()), 
            None => return Err(()), 
        }
    }

    fn get_star_2(input: &str) -> Result<i64, ()> {
        match Day6::pooled_decode(input.as_bytes(), 14, 8) {
            Some(v) => return Ok(v.try_into().unwrap()), 
            None => return Err(()), 
        }
    }
}

impl Day6 {
    fn pooled_decode(input: &[u8], config: usize, thread_count: usize) -> Option<usize> {
        let mut pooled_decoder = PooledDecoder::new(input.to_vec(), config, thread_count); 
        return pooled_decoder.execute(); 
    }

    // Actually all-diff, refactor later
    fn is_packet_start_marker(u8_subslice: &[u8]) -> bool {
        let hs: HashSet<u8> = HashSet::from_iter(u8_subslice.iter().cloned()); 
        if hs.len() == u8_subslice.len() { true } else { false }
    }  
}

#[cfg(test)]
mod tests {
    use crate::get_solutions::AOCSolutions;

    use super::Day6; 

    const SAMPLE_1: &str = "wxzy"; // 4
    const SAMPLE_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz"; // 5
    const SAMPLE_3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"; // 10
    const SAMPLE_4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"; // 11

    #[test]
    fn test_invariance_between_thread_count() {
        assert_eq!(Day6::pooled_decode(SAMPLE_1.as_bytes(), 4, 4), Day6::pooled_decode(SAMPLE_1.as_bytes(), 4, 1));
        assert_eq!(Day6::pooled_decode(SAMPLE_2.as_bytes(), 4, 4), Day6::pooled_decode(SAMPLE_2.as_bytes(), 4, 1));
        assert_eq!(Day6::pooled_decode(SAMPLE_3.as_bytes(), 4, 4), Day6::pooled_decode(SAMPLE_3.as_bytes(), 4, 12));
        assert_eq!(Day6::pooled_decode(SAMPLE_4.as_bytes(), 4, 4), Day6::pooled_decode(SAMPLE_4.as_bytes(), 4, 7));
    }

    #[test]
    fn test_results() {
        assert_eq!(Day6::pooled_decode(SAMPLE_1.as_bytes(), 4, 4).unwrap(), 4); 
        assert_eq!(Day6::pooled_decode(SAMPLE_2.as_bytes(), 4, 8).unwrap(), 5);
        assert_eq!(Day6::pooled_decode(SAMPLE_3.as_bytes(), 4, 12).unwrap(), 10);
        assert_eq!(Day6::pooled_decode(SAMPLE_4.as_bytes(), 4, 1).unwrap(), 11); 
    }

    #[test]
    fn test_get_star_2() {
        assert_eq!(Day6::get_star_2(SAMPLE_2).unwrap(), 23); 
    }
}