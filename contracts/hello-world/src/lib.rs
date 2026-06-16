#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol, Address};

// 1. Định nghĩa cấu trúc dữ liệu cho Gói Đăng Ký (Subscription Plan)
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Plan {
    pub id: u32,
    pub price: i128,       // Giá gói
    pub duration: u64,    // Thời hạn gói (tính bằng giây, ví dụ: 30 ngày = 2592000)
}

// 2. Định nghĩa cấu trúc dữ liệu Đăng Ký của Người Dùng (User Subscription)
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Subscription {
    pub plan_id: u32,
    pub start_time: u64,
    pub end_time: u64,
    pub auto_renew: bool,
    pub is_active: bool,
}

// Biến đếm ID gói đăng ký lưu trên hệ thống
const PLAN_COUNTER: Symbol = symbol_short!("PLN_CNT");

#[contract]
pub struct ChainSubscriptionHub;

#[contractimpl]
impl ChainSubscriptionHub {

    // Chức năng 1: Tạo một Gói Đăng Ký mới (Ai cũng có thể tạo để thử nghiệm nhanh)
    pub fn create_plan(env: Env, price: i128, duration: u64) -> u32 {
        // Tự động tăng ID gói
        let mut current_id: u32 = env.storage().instance().get(&PLAN_COUNTER).unwrap_or(0);
        current_id += 1;

        let plan = Plan {
            id: current_id,
            price,
            duration,
        };

        // Lưu gói vào Blockchain (Key là ID gói)
        env.storage().instance().set(&current_id, &plan);
        env.storage().instance().set(&PLAN_COUNTER, &current_id);

        current_id
    }

    // Chức năng 2: Người dùng đăng ký gói (User Subscriptions)
    pub fn subscribe(env: Env, user: Address, plan_id: u32, auto_renew: bool) -> bool {
        user.require_auth();

        // Kiểm tra xem gói đăng ký (plan_id) này có tồn tại không
        let plan: Plan = match env.storage().instance().get(&plan_id) {
            Some(p) => p,
            None => panic!("Subscription plan does not exist"),
        };

        // Lấy thời gian hiện tại của block (Unix Timestamp)
        let now = env.ledger().timestamp();
        let end_time = now + plan.duration;

        let sub = Subscription {
            plan_id,
            start_time: now,
            end_time,
            auto_renew,
            is_active: true,
        };

        // Lưu thông tin đăng ký của người dùng (Key là Địa chỉ ví)
        env.storage().instance().set(&user, &sub);

        true
    }

    // Chức năng 3: Hủy gói (Subscription Cancellation)
    pub fn cancel_subscription(env: Env, user: Address) -> bool {
        user.require_auth();

        let mut sub: Subscription = match env.storage().instance().get(&user) {
            Some(s) => s,
            None => return false,
        };

        // Tắt hoạt động
        sub.auto_renew = false;
        sub.is_active = false;

        env.storage().instance().set(&user, &sub);
        true
    }

    // Chức năng 4: Tra cứu trạng thái đăng ký (Transparent Status)
    pub fn get_subscription(env: Env, user: Address) -> Option<Subscription> {
        env.storage().instance().get(&user)
    }
}