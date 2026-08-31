---
status: accepted
---

# Composer statiquement sans runtime obligatoire

Meta-system est d’abord un framework Rust de composition statique: il valide les Addon Contracts et
leurs Capabilities pendant la compilation, puis laisse le programme obtenu s’exécuter normalement.
Le prototype de Kernel dynamique a démontré qu’Events, cycles de vie, routage et chargement natif
forment un modèle cohérent, mais les rendre universels éloigne le projet de sa valeur centrale de
capitalisation; un runtime ou une composition dynamique pourront donc être apportés ultérieurement
par des Addons sans entrer dans le noyau.
