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

* **Oʻrnatuv:** [Oʻzbekcha oʻrnatuv qoʻllanmasi](https://github.com/tilsevarlar-gurungu/latinga/blob/main/docs/uz/OERNATUV.md).
* **Koʻrgazma:** Ilovani [tilsevarlar-gurungu.github.io/latinga/](https://tilsevarlar-gurungu.github.io/latinga/) jonli veb sahifasida sinab koʻring.
* [English version of README](https://github.com/tilsevarlar-gurungu/latinga/blob/main/README_EN.md)

## Tezkor Foydalanish

```
# Faylni yangi imloga oʻgirish (Kelgusi tartib)
$ latinga matn.txt
```

```
# STDIN orqali amaldagi imloga oʻgirish (Joriy tartib)
$ echo "кирилл" | latinga --joriy
```

```
# Matndagi imlo xatolarini tekshirish
$ latinga -t matn.txt
```
