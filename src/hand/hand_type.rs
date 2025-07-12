#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    RoyalFlush,
    StraightFlush,
    FourKind,
    FullHouse,
    Flush,
    Straight,
    ThreeKind,
    TwoPair,
    Pair,
    HighCard,
    None,
    Deck,
}
