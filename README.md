# Chronos: Sistem de Ingerare și Analiză Polymarket

Chronos este un sistem de înaltă performanță proiectat pentru a urmări în timp real datele de tranzacționare de pe Polymarket CLOB (Central Limit Order Book). Acesta este optimizat pentru a rula pe hardware de tip Edge (precum **NVIDIA Jetson Orin Nano**) folosind un NVMe extern pentru persistență bazată pe **TimescaleDB**.

## 🚀 Arhitectură

Sistemul este împărțit în trei componente principale:

1.  **Ingestion Service (Rust):** Un serviciu asincron construit cu `tokio` și `tokio-tungstenite` care se conectează la WebSocket-ul Polymarket. Acesta ascultă evenimentele de tip `price_change` sau `last_trade_price` și le salvează instantaneu în baza de date.
2.  **Database (TimescaleDB/PostgreSQL):** Rulează în Docker și folosește extensia TimescaleDB pentru a optimiza stocarea seriilor temporale (time-series). Datele sunt stocate într-un `hypertable` pentru interogări ultra-rapide.
3.  **Analytics API (Python/FastAPI):** Un server web care oferă endpoint-uri pentru calcularea indicatorilor tehnici, cum ar fi **Media Mobilă (Rolling Moving Average)** pe o oră, folosind `pandas` pentru procesare eficientă.

## 🛠️ Instalare

### Cerințe minime
*   **Docker** și **Docker Compose**
*   **Rust** (cargo)
*   **Python 3.10+**
*   Acces SSH la dispozitivul Jetson (dacă rulați remote)

### Pași pentru pornire rapidă

1.  **Clonează repozitoriul:**
    ```bash
    git clone https://github.com/arhimonde/Chronos.git
    cd Chronos
    ```

2.  **Configurare:**
    Deschide `ingestion/src/websocket.rs` și modifică ID-urile piețelor pe care dorești să le urmărești în variabila `assets_ids`.

3.  **Lansare automată:**
    Scriptul `RUN_PROJECT.sh` se ocupă de pornirea bazei de date, crearea mediului virtual Python și compilarea serviciului Rust:
    ```bash
    chmod +x RUN_PROJECT.sh
    ./RUN_PROJECT.sh
    ```

## 📈 Utilizare

### Monitorizarea Ingerării
Odată pornit, vei vedea în terminal log-uri de tipul:
`[INFO] Saved Trade -> Market: 0x... | Outcome: ... | Price: $...`

### Accesarea Analizelor (API)
API-ul rulează implicit pe portul `8000`. Poți accesa interfața interactivă Swagger aici:
👉 `http://<IP_JETSON>:8000/docs`

Folosește endpoint-ul `/api/markets/{market_id}/trends` pentru a vedea evoluția prețului și media mobilă.

## 🗄️ Structura Bazei de Date
Tabelul `price_history` conține:
*   `timestamp`: Timpul exact al tranzacției (UTC).
*   `market_id`: Adresa contractului pieței.
*   `outcome`: Rezultatul pariat (ex: "Yes", "No").
*   `price`: Prețul (odds) la momentul respectiv.
*   `size`: Volumul tranzacției.

## 🔧 Depanare
*   **Erori WebSocket:** Polymarket resetează conexiunile inactive. Scriptul de Rust are logică de **auto-reconnect** integrată (reîncearcă la fiecare 5 secunde).
*   **Lipsă date în API:** Verifică dacă în terminal apar log-uri cu "Saved Trade". Dacă nu apar, înseamnă că piața aleasă nu are tranzacții active în acest moment.

---
Proiect dezvoltat pentru monitorizare predictivă și trading algoritmic.
