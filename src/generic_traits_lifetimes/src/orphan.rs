use aggregator::SocialPost;
use std::fmt;

// this is to demonstrate what an orphan looks like
// recall that an orphan is a trait impl that violates the orphan rule
// a compiler constraint ensuring that either the trait or the type being impl
// is local to your current crate

impl fmt::Display for SocialPost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Post by {}", self.username)
    }
}
