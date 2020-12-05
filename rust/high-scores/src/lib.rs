#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a[u32],
}

impl <'a> HighScores <'a >{
    pub fn new(scores: &'a[u32]) -> Self {
        Self {scores}
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.last() {
            Some(pointer) => {
                Some(*pointer)
            }
            None => None
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores == vec![] {
            None
        } else {
            let mut best: u32 = 0;
            self.scores.iter().for_each(|x| if x >= &best { best = *x} );
            Some(best)
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        if self.scores.len() > 3 {
            let mut result = self.scores.to_vec();
            result.sort_by(|a, b| b.cmp(a)) ;
            result[..3].to_vec()
        } else if self.scores.len() > 0 {
            let mut result = self.scores.to_vec();
            result.sort_by(|a, b| b.cmp(a)) ;
            result
        } else {
            vec![]
        }
    }
}
