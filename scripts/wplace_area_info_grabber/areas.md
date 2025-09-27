`areas` è l'input per questo programma
- Linea 1: Commento
- Linea 2: Azioni 
- Linea 3: Specifica il punto in alto a sinistra dell'area
- Linea 4: Dipende dalla Linea 2, l'azione
  * Se la Linea 2 è `T`, allora va lasciata vuota, il programma capirà il template a cui fa riferimento dal punto in alto a sinistra e controllerà solo i pixel dell'art
  * Se la Linea 2 è `P`, allora va specificato il punto in basso a destra dell'art
  
Le azioni:
- `T`: Template, controlla la Linea 4
- `P`: Punti, controlla la Linea 4
- `D`: Scarica l'area in un file `input_n.png`
- `H`: Crea una sorta di heatmap in `heatmap_n.png`