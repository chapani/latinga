# Latinga 🇺🇿

Latinga — Rust tilida yozilgan, kengayuvchan va yuqori samaradorlikka ega oʻzbekcha kirill-lotin harf oʻgiruvchisidir. U katta hajmdagi matnlar va texnik hujjatlar (LaTeX, HTML, Markdown) bilan ishlashga moʻljallangan.

## Talaffuzi

Latinga — /loˈtiŋɡa/ yani "lotinga" deb oʻqiladi. Chunki umid shulki kelajakda Oʻrtoq Turk Alifbosida yozamiz. 😊

## Asosiy Xususiyatlar

- **Aqlli Qalqon**: LaTeX formulalari, HTML tamgʻalari va Markdown bitik parchalarini avtomatik taniydi va ularni oʻzgartirmaydi.
- **Ikki xil Imlo**: Ham **Joriy** (sh, ch, oʻ, gʻ), ham **Kelgusi** (ş, ç, ö, ğ) standartlarini qoʻllab-quvvatlaydi.
- **Tutuq bilan ajratilgan atoqli otlar**: Kelgusi imloda atoqli otlardan kelishik qoʻshimchalarini avtomatik ajratadi (masalan: `Toshkentda` -> `Toşkent'da`).
- **Yagona H**: Kelgusi imloda faqat bitta H qoladi va X harfi H bilan almashtiriladi (masalan: `Xato` -> `Hato`).
- **Hujjat turlari**: ilova LaTeX, HTML, XML, Markdown hujjat turlari bilan ishlay oladi.

## 🚀 Foydali Bogʻlamalar

* **Oʻrnatuv:** [Oʻzbekcha oʻrnatuv qoʻllanmasi](docs/uz/OERNATUV.md).
* **Koʻrgazma:** Ilovani [chapani.github.io/latinga/](https://chapani.github.io/latinga/) jonli veb sahifasida sinab koʻring.

## Tezkor Foydalanish

```
# Faylni yangi imloga oʻgirish (Kelgusi tartib)
$ latinga matn.txt
```

```
# STDIN orqali amaldagi imloga oʻgirish
$ echo "кирилл" | latinga --joriy
```

```
# Matndagi imlo xatolarini tekshirish
$ latinga -s matn.txt
```
