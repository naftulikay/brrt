mod brrt;

use brrt::repository::load_config;
use brrt::repository::config::dothings;

fn main() {
    load_config();
    dothings();
}
