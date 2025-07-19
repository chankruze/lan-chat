### 🔁 **mDNS Basics Recap**

- mDNS (Multicast DNS) lets devices on the same local network discover each other **without needing a central server**.
- For example: Your LAN chat app can say, “Hey, I'm here! My name is `Chandan's Chat` and I'm at IP `192.168.1.15` on port `8080`.”

### ✅ **Multicast Response**

- **Default behavior in standard mDNS.**
- When someone sends a query like:
  ➤ “Anyone offering `_chat._tcp.local`?”
- Then **all matching devices respond back to everyone on the multicast address** (like a group reply).
- Everyone on the network sees the response.

### ❌ **Unicast-Only Response (in `librespot/mdns`)**

- The crate **only sends responses back to the _sender_** of the query, not the whole multicast group.
- That means:

  - The **peer that sends a discovery request gets a response**.
  - But **other peers don't learn about each other unless they also initiate a query**.

> Think of it like this:
>
> - Multicast response: you shout, others shout back, and _everyone hears_.
> - Unicast response: you whisper a question to the room, and only you get whispered answers.

### 📌 **Why it matters**

- For most peer discovery use cases like LAN chat, **unicast is perfectly fine**.
- Each device just needs to **scan (`discover::all`)**, and will learn about available peers.
- But for a full broadcast-style announcement (like Apple's Bonjour), multicast response support is needed.
