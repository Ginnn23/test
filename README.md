# ChainSubscription Hub 🚀

## Project Description
ChainSubscription Hub is a decentralized smart contract platform designed to manage subscription plans and user subscriptions with automated renewals. Built using Soroban on the Stellar blockchain, it provides transparent management by enforcing subscription rules and renewal cycles, while allowing users to pause or cancel their subscriptions securely and trustlessly.

## Project Vision
The vision of ChainSubscription Hub is to offer subscription-based businesses a decentralized, secure, and automated way to handle user subscriptions and renewals without relying on centralized intermediaries. This approach guarantees transparent access management and payment enforcement, ultimately increasing user trust and business reliability.

## Key Features
* **Plan Management:** Admins can easily create and manage subscription plans, specifying duration and pricing.
* **User Subscriptions:** Users can subscribe to active plans with flexible options to enable auto-renewal.
* **Subscription Cancellation:** Users maintain full control and can cancel their subscriptions, disabling immediate or future renewals at any time.
* **Immutable Records:** All subscription plans and user states are securely recorded on-chain for complete auditability.
* **Transparent Status:** Subscription statuses and historical data are publicly accessible, ensuring maximum transparency.

## Usage Instructions (Giao diện UI / Test)
1. **Create Plans:** Tìm đến hàm `create_plan`, nhập giá (`price`) và thời gian (`duration` tính bằng giây) rồi bấm nút Call để tạo gói. Gói đầu tiên sẽ có ID là `1`.
2. **Subscribe:** Người dùng nhập địa chỉ ví của mình vào ô `user`, điền `plan_id` là `1`, và chọn `auto_renew` (true/false) rồi bấm Call để đăng ký.
3. **Cancel:** Người dùng muốn hủy gói chỉ cần nhập địa chỉ ví vào hàm `cancel_subscription` và bấm Call.
4. **Query:** Nhập địa chỉ ví vào hàm `get_subscription` để kiểm tra thời gian kích hoạt và trạng thái gói thực tế trên mạng lưới.

## Technology Stack
* **Rust & Soroban SDK:** Utilized for secure, highly optimized smart contract development.
* **Stellar Blockchain:** Serves as the robust foundation for decentralized, immutable state management.

## License
This project is licensed under the **MIT License**.

 [screenshot] (https://ibb.co/mF1BT49N)
---

### Contract Detail
* **Network:** Stellar / Soroban Testnet
* **Contract ID:** CCX3J5B2JJIZQHEPV4PSLVM5EVQQLQL6F774L4BF6VNXZKQ3ROFSNR4H
