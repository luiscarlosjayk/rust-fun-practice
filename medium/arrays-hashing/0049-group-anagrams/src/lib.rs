//! See README.md for the problem statement.

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut words_map: std::collections::HashMap<[u8; 26], Vec<String>> =
        std::collections::HashMap::new();

    for s in strs {
        let mut counts = [0u8; 26];
        for b in s.bytes() {
            counts[(b - b'a') as usize] += 1;
        }

        words_map.entry(counts).or_default().push(s);
    }

    words_map.into_values().collect()
}

/*
 * En la línea 10:
 
   counts[(b - b'a') as usize] += 1;
 
   Se resta b'a' para convertir el código del carácter en un índice de 0 a 25 dentro del array counts: [u8; 26].
 
   El razonamiento
 
   Cada letra es un byte según su valor ASCII:
 
   ┌───────┬──────────────┐
   │ Letra │ Byte (ASCII) │
   ├───────┼──────────────┤
   │ 'a'   │ 97           │
   ├───────┼──────────────┤
   │ 'b'   │ 98           │
   ├───────┼──────────────┤
   │ 'c'   │ 99           │
   ├───────┼──────────────┤
   │ ...   │ ...          │
   ├───────┼──────────────┤
   │ 'z'   │ 122          │
   └───────┴──────────────┘
 
   El array counts tiene solo 26 posiciones (índices 0–25), una por cada letra. Pero los bytes de las letras van de 97 a 122. Si
   usaras el byte directamente, counts[97] se saldría del array (panic por índice fuera de rango).
 
   Al restar b'a' (que es 97), normalizas el rango:
 
   - 'a' → 97 - 97 = 0
   - 'b' → 98 - 97 = 1
   - 'c' → 99 - 97 = 2
   - ...
   - 'z' → 122 - 97 = 25
 
   Así cada letra cae justo en su casilla correspondiente, y counts termina siendo un histograma de frecuencias de las letras de
   la palabra.
 
   Por qué sirve para anagramas
 
   Dos palabras son anagramas si tienen exactamente las mismas letras con las mismas cantidades. Como counts captura
   precisamente eso, dos anagramas producen el mismo array [u8; 26], que se usa como clave del HashMap para agruparlos.
 
   "eat" → [a:1, e:1, t:1, ...resto 0]
   "tea" → [a:1, e:1, t:1, ...resto 0]   // misma clave → mismo grupo
 
   Una nota: esto asume que la entrada solo contiene minúsculas a–z (lo cual garantizan las restricciones de este problema de
   LeetCode). Si llegara una mayúscula o un espacio, b - b'a' daría un índice incorrecto o causaría un panic.
 */