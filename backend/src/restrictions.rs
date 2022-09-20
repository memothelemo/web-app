use rocket_governor::{Method, Quota, RocketGovernable, RocketGovernor};

pub type RateLimit<'a> = RocketGovernor<'a, RateLimitGuard>;
pub struct RateLimitGuard;

impl<'r> RocketGovernable<'r> for RateLimitGuard {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        Quota::per_second(Self::nonzero(3u32))
    }
}
