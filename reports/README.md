# Raporty — Projekt 1

## Personalizacja

Edytuj wyłącznie `shared/report-resources.tex`, aby ustawić:

- imię i nazwisko,
- numer studenta,
- uczelnię i wydział/kierunek,
- miejsce oraz datę.

Zmiany będą użyte we wszystkich czterech raportach.

## Kompilacja

Uruchamiaj kompilację z katalogu danego raportu, np.:

```bash
cd report-1
latexmk -pdf main.tex
```

Wymagane są standardowe pakiety TeX Live, w szczególności `babel-polish`,
`algorithm`, `algpseudocode` i `cleveref`.

## Zasady stylu

- A4, margines lewy 30 mm, pozostałe marginesy 25 mm.
- Dokumenty są po polsku i używają `report` jako klasy dokumentu.
- Wspólna typografia, nagłówki, stopki, podpisy, listingi i pseudokod są w
  `shared/report-style.sty`.
- Wspólne dane i teksty są w `shared/report-resources.tex`.
- Wspólna baza BibTeX jest w `shared/references.bib`. Dodawaj `\cite{klucz}`
  tylko w raporcie, w którym rzeczywiście używasz danego źródła; bibliografia
  każdego raportu zostanie wtedy utworzona osobno.
- Cytowania używają stylu numerycznego IEEE (`IEEEtran`).
- Diagramy można rysować bezpośrednio w LaTeX-u przy użyciu TikZ. Wspólne style
  `graph vertex`, `graph edge`, `graph arrow`, `diagram box` i `diagram arrow`
  są zdefiniowane w `shared/report-style.sty`. Przykład znajduje się w
  `report-1/figures/graph-types.tex`.
- Nie należy kopiować preambuły do poszczególnych raportów.
