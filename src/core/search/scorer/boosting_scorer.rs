use core::search::scorer::Scorer;
use core::search::DocIterator;
use core::util::DocId;
use error::Result;

pub struct BoostingScorer {
    positive: Box<dyn Scorer>,
    negative: Box<dyn Scorer>,
    negative_boost: f32,
}

impl BoostingScorer {
    pub fn new(positive: Box<dyn Scorer>, negative: Box<dyn Scorer>, negative_boost: f32) -> BoostingScorer {
        debug_assert!(negative_boost > 0.0 && negative_boost < 1.0);
        BoostingScorer {
            positive,
            negative,
            negative_boost,
        }
    }
}

impl Scorer for BoostingScorer {
    fn score(&mut self) -> Result<f32> {
        let current_doc = self.positive.doc_id();
        let mut score = self.positive.score()?;

        if current_doc == self.negative.advance(current_doc)? {
            score *= self.negative_boost;
        }

        Ok(score)
    }
}

impl DocIterator for BoostingScorer {
    fn doc_id(&self) -> DocId {
        self.positive.doc_id()
    }

    fn next(&mut self) -> Result<DocId> {
        self.positive.next()
    }

    fn advance(&mut self, target: DocId) -> Result<DocId> {
        self.positive.advance(target)
    }

    fn cost(&self) -> usize {
        self.positive.cost()
    }

    fn matches(&mut self) -> Result<bool> {
        self.positive.matches()
    }

    fn approximate_next(&mut self) -> Result<DocId> {
        self.positive.approximate_next()
    }

    fn approximate_advance(&mut self, target: DocId) -> Result<DocId> {
        self.positive.approximate_advance(target)
    }
}