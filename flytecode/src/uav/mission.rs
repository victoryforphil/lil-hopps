
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum UAVMission{
    Takeoff,
    Land,
    Hover,
    MoveTo,
    Debug,
    Test,
    Smoketest
}