PhoneLedger — Inventario de Teléfonos en Solana

PhoneLedger es un programa on-chain construido en Rust utilizando el framework Anchor sobre la blockchain de Solana. Su objetivo es permitir que una tienda de celulares o negocio tecnológico registre y administre su inventario de teléfonos de forma descentralizada, segura y permanente en la blockchain.

Este sistema guarda la información directamente en cuentas del programa, lo que significa que los datos no dependen de un servidor tradicional.

Objetivo del proyecto

PhoneLedger funciona como un sistema de gestión de inventario en blockchain que permite realizar operaciones básicas sobre teléfonos registrados.

Las principales funciones del sistema son:

• Crear el perfil de una tienda de teléfonos asociado a una wallet
• Registrar nuevos teléfonos con su información
• Actualizar información del inventario
• Activar o desactivar disponibilidad de un teléfono
• Eliminar teléfonos del sistema

Toda la información queda registrada de manera transparente, segura e inmutable en la red de Solana.

Estructura del programa

El programa organiza los datos en una jerarquía simple dentro de la blockchain.

Wallet (Owner)
│
└── Cuenta de Tienda de Teléfonos
│
├── Teléfono 1
├── Teléfono 2
└── Teléfono 3

Cada tienda tiene su propio inventario vinculado a la wallet del propietario.

Estructuras de datos principales
Tienda de Teléfonos

Campo | Tipo | Descripción
owner | Pubkey | Dirección de la wallet propietaria
nombre | String | Nombre del negocio o tienda
inventario | Vec | Lista de teléfonos almacenados

Teléfono

Campo | Tipo | Descripción
nombre | String | Nombre del modelo de teléfono
marca | String | Marca del dispositivo
precio | u32 | Precio del teléfono
activo | bool | Indica si está disponible en el inventario

Funciones del programa

El contrato inteligente incluye varias instrucciones para interactuar con los datos:

crear_tienda(nombre)
Crea una nueva cuenta de tienda vinculada al propietario.

registrar_telefono(nombre, marca, precio)
Agrega un teléfono nuevo al inventario.

actualizar_precio(nombre, nuevo_precio)
Permite modificar el precio de un teléfono existente.

cambiar_estado(nombre)
Activa o desactiva la disponibilidad de un teléfono.

eliminar_telefono(nombre)
Remueve un teléfono del inventario almacenado.

Direcciones derivadas (PDA)

El programa utiliza Program Derived Addresses (PDA) para generar cuentas únicas dentro de Solana.

Cuenta | Seeds utilizadas
Tienda | ["tienda_telefonos", owner_pubkey]

Esto asegura que:

• Cada wallet puede tener solo una tienda registrada
• Los datos no pueden ser modificados por terceros
• Solo el propietario de la cuenta puede actualizar su inventario

Cómo ejecutar el proyecto

Para probar el programa puedes utilizar Solana Playground.

Pasos básicos:

Abrir Solana Playground

Crear o pegar el código dentro del archivo src/lib.rs

Conectar tu wallet en la red Devnet

Presionar Build para compilar el programa

Presionar Deploy para publicarlo

Ejecutar las funciones desde el panel de pruebas

Ejemplo de uso

Flujo típico de interacción con el programa:

crear_tienda("Tech Mobile Store")

registrar_telefono("iPhone 15", "Apple", 25000)

registrar_telefono("Galaxy S24", "Samsung", 23000)

cambiar_estado("iPhone 15")

actualizar_precio("Galaxy S24", 24000)

eliminar_telefono("iPhone 15")

Tecnologías utilizadas

Tecnología | Uso dentro del proyecto
Solana | Blockchain donde se ejecuta el programa
Anchor | Framework para desarrollar smart contracts
Rust | Lenguaje principal del programa

Autor

Proyecto desarrollado como parte de la certificación de Solana, implementando un smart contract en Solana para la gestión descentralizada de inventario de teléfonos.
