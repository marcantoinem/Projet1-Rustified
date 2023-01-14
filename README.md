# Projet 1 - Rust
Ce projet est une version entièrement écrite en Rust du [cours INF1900](https://cours.polymtl.ca/inf1900/). Le premier but de ce repo est de montrer qu'il est entièrement possible et même pas si complexe de faire du Rust embarqué pour le projet 1.
## Librairies utilisées
Les librairies [avr-hal](https://github.com/marcantoinem/avr-hal) et [avr-device](https://github.com/marcantoinem/avr-device) ont été patchés pour fonctionner avec le ATMEGA324pa. Les patchs ont été reportés en upstream et vont probablement être merge dans une prochaine version de `avr-device` et `avr-hal`.