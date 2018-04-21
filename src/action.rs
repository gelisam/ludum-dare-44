use hex::HexVector;


#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
enum Action {
    Move(HexVector),
    GrabPowerup(HexVector),
    PlaceBomb(HexVector),
    Kick(HexVector),
    Zip(HexVector),
    ActivateSpikeShield,
}
