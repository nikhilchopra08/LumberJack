# 🌲 Solana Lumberjack Program 🌲  

A simple on-chain game where players chop trees to collect wood using energy that regenerates over time. Built with **Anchor** for Solana.  

---

## 📜 Program Overview  

This program allows players to:  
✅ **Initialize** their player account  
✅ **Chop trees** to earn wood (consumes energy)  
✅ **Regenerate energy** automatically over time  

---

## ⚙️ Core Functions  

### 1. `init_player`  
Initializes a new player account with:  
- Custom `name`  
- Starting energy: `5/5`  
- Wood: `0`  

```rust  
pub fn init_player(ctx: Context<InitPlayer>, name: String) -> Result<()>  
```  

### 2. `chop_tree`  
Chops a tree to earn wood:  
- Costs **1 energy** per chop  
- Gains **1 wood** per successful chop  
- Fails if energy is `0`  

```rust  
pub fn chop_tree(mut ctx: Context<ChopTree>) -> Result<()>  
```  

---

## 🔋 Energy System  
- **Max Energy**: `5`  
- **Recharge Rate**: `1 energy / 30 seconds`  
- **Auto-Update**: Energy is calculated based on time passed since last action  

---

## 📦 Player Data Structure  
Stored on-chain for each player:  

| Field         | Type   | Description                     |  
|---------------|--------|---------------------------------|  
| `name`        | String | Player's chosen name           |  
| `level`       | u8     | (Reserved for future use)      |  
| `xp`          | u64    | (Reserved for future use)      |  
| `wood`        | u64    | Total wood collected           |  
| `energy`      | u64    | Current energy (0-5)           |  
| `last_login`  | i64    | Timestamp of last action       |  

---

## 🚧 Error Handling  
- `NotEnoughEnergy` → Trying to chop with `0 energy`  

---

## 🛠️ Usage Example  

1. **Initialize Player**  
```rust  
init_player("Woody")  
```  

2. **Chop Trees**  
```rust  
chop_tree() // +1 wood, -1 energy  
```  

3. **Wait for Energy Recharge**  
Energy refills automatically!  

---

## 📍 Program ID  
```  
94N66D6gFXeFVFphMi8dGJsdxU3QdAAcy64ZqFBqag6g  
```  

Built with 🪓 and ❤️ using Anchor on Solana!  

--- 

Let me know if you'd like any modifications! This keeps it clean while highlighting the key features.
