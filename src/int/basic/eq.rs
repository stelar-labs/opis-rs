use crate::Int;

impl PartialEq for Int {

    fn eq(&self, b: &Self) -> bool {
        
        self.bits == b.bits
    
    }
}

impl Eq for Int {}
