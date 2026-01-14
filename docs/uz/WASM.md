# Latinga va WebAssembly (WASM)

Latinga'ni veb-ilovalar yoki Node.js muhitiga yuqori tezlikdagi kutubxona sifatida qoʻshish mumkin. ```Latinga``` ob'ekti orqali lugʻatlarni bir marta xotiraga yuklab, ulardan samarali foydalanish imkoniyati mavjud.

## Oʻrnatish
Avval ```wasm-pack``` yordamida loyihani yigʻing:
```bash
wasm-pack build --features wasm
```

## Foydalanish Namunalari

### 1. Brauzerda (ES Modules)
```javascript
import init, { Latinga } from './pkg/latinga.js';

async function run() {
    await init();
    
    // Router obyektini yaratish (Bir marta bajariladi)
    // false = Kelgusi imlo, true = Joriy imlo
    const latinga = new Latinga(false);

    // Maxsus qoidalar qoʻshish
    latinga.almashuvchilarni_yukla("Apple:Epl;Google:Gugl");
    latinga.atoqlilarni_yukla("Toshkent,Samarqand");
    latinga.qalqonlarni_yukla("\\{.*?\\}"); // Jingalak qavslarni himoyalash

    // Oʻgirish (Juda tez bajariladi)
    const natija = latinga.ogir("Toshkentga bordik.");
    console.log(natija); // Toşkent'ga bordik.
}
```

### 2. Node.js muhitida
Node.js uchun yigʻilgan paketdan quyidagicha foydalaniladi:

```javascript
const { Latinga } = require('./pkg/latinga.js');

const latinga = new Latinga(true); 
console.log(latinga.ogir("Ўзбекистон")); 
```

## Texnik Tavsiya
```Latinga``` yaratilganda ichki lugʻatlar xotiraga yuklanadi. Shu sababli, bitta latingadan butun dastur davomida foydalanish tavsiya etiladi.
