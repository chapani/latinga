# Latinga: Foydalanish Boʻyicha Toʻliq Qoʻllanma

Latinga oʻzbek lotin alifbosiga oʻgiruvchi, buyruq satri ilovasidir. Ushbu qoʻllanmada asosiy imkoniyatlar misollar bilan tushuntiriladi.

## 1. Imlo Tartiblari

Dastur ikki xil tartibda ishlaydi:

| Tartib | Bayroq | Misol (Kirill -> Lotin) |
| :--- | :--- | :--- |
| **Kelgusi** | Fitriy | шаҳар -> şahar, ўрдак -> ördak |
| **Joriy** | -j, --joriy | шаҳар -> shahar, ўрдак -> oʻrdak |


## 2. Atoqli Otlar va Qoʻshimchalar (-a, --atoqli)

Kelgusi imloda atoqli otlardan keyin keladigan kelishik qoʻshimchalari tutuq belgisi (') bilan ajratilishi kerak.

```
$ latinga -a "Samarqand,Termiz"
# Kiritma: "Termizga Samarqanddan boramiz."
# Chiqarma: "Termiz'ga Samarqand'dan boramiz."
```

## 3. Matnni Qalqonlash

Texnik hujjatlar (bitiklar, formulalar) oʻzgarib ketmasligi uchun Latinga bir necha xil himoya usulidan foydalanadi.

### A. Avtomatik Qalqon

Dastur quyidagilarni avtomatik taniydi:

- **LaTeX**: $f'(x) = y$ va \cite{shahar2024} oʻzgarmaydi.
- **HTML**: usttamgʻalar ichidagi content oʻgiriladi, lekin tamgʻaning oʻzi himoyalanadi.
- **Email**: info@latinga.uz kabi manzillar oʻzgarmaydi.
- **XML**: bu format juda egiluvchan boʻlganligi uchun, cheklangan qoʻllovga ega.

### B. Umumiy Himoya {] ... [}

Agar matnning biror qismi mutloq oʻzgarmasligini xohlasangiz:

- Kiritma: Ер ва {]Ер[}.
- Chiqarma: Yer va Ер.

### C. Qolipli Qalqon (-n, --qalqon)

Murakkab qoliplarni oʻzgarishdan qalqonlash ya'ni oʻzgarmasdan qolishini ta'minlash uchun ishlatiladi:

```
$ latinga matn.txt -n "\*\*Ruscha:\*\* ([^\n]+)"
```

## 4. Maxsus Almashtirish (-m, --almashtir)

Istalgan soʻzni boshqasi bilan almashtiradi:

```
# Nuqta-vergul (;) bilan bir nechta qoida berish
$ latinga matn.txt -m "октябрь:oktabr;сентябрь:sentabr"

# Yoki qoidalar yozilgan faylni koʻrsatish
$ latinga matn.txt -m almashtiruvlar.txt
```

## 5. Sinab koʻrish

Matndagi tutuq belgisi (joriy tartibda) va eskirgan harf mavjudligini tekshiradi (kelgusi tartibda).
```
$ latinga -s matn.txt
```

Agar xato topilsa, dastur xato satri va ustunini koʻrsatadi hamda xato holati boʻlgan `exit status 1` bilan tugaydi.
