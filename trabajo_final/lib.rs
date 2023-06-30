#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(unused)]
pub use self::trabajo_final::ClubRef;
mod fecha;
#[ink::contract]
mod trabajo_final {   
    
    use crate::fecha::Fecha;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    /*
    Notas:
        id_socio, en cualquier contexto, es el índice del socio en el vector de socios (empieza en 0)
    
    */

    #[ink(storage)]    
    pub struct Club {
        nombre: String,
        pagos: Vec<Pago>,
        socios: Vec<Socio>,
        // Precio de cada categoría, en tokens por mes
        precio_cat_a: u128,
        precio_cat_b: u128,
        precio_cat_c: u128,
        // cantidad_pagos: u128,
        cantidad_pagos_bonificacion: u128,
        porcentaje_bonificacion: u128,
        // permisos, etc.
    }

    impl Club {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                nombre: "Seminario Rust".into(),
                pagos : Vec::new(),
                socios: Vec::new(),
                precio_cat_a: 5000,
                precio_cat_b: 3000,
                precio_cat_c: 2000,
                // cantidad_pagos: 0,
                cantidad_pagos_bonificacion:5,
                porcentaje_bonificacion: 10
            }
        }

        #[ink(message)]
        pub fn cambiar_nombre(&mut self, nuevo_nombre: String) {
            self.nombre = nuevo_nombre;
        }

        #[ink(message)]
        pub fn get_nombre(&self) -> String {
            self.nombre.clone()
        }

        #[ink(message)]
        pub fn set_precio_cat_a(&mut self, nuevo_valor:u128) {
            self.precio_cat_a = nuevo_valor;
        }

        #[ink(message)]
        pub fn set_precio_cat_b(&mut self, nuevo_valor:u128) {
            self.precio_cat_b = nuevo_valor;
        }

        #[ink(message)]
        pub fn set_precio_cat_c(&mut self, nuevo_valor:u128) {
            self.precio_cat_c = nuevo_valor;
        }

        #[ink(message)]
        pub fn obtener_precio(&self, categoria: Categoria) -> u128 {
            match categoria {
                Categoria::CategoriaA => self.precio_cat_a,
                Categoria::CategoriaB => self.precio_cat_b,
                Categoria::CategoriaC => self.precio_cat_c,
            }
        }

        #[ink(message)]
        pub fn set_bonificacion_pagos_consecutivos(&mut self, nuevo_valor:u128) {
            self.cantidad_pagos_bonificacion = nuevo_valor;
        }

        #[ink(message)]
        pub fn set_porcentaje_bonificacion_pagos_consecutivos(&mut self,nuevo_valor:u128) {
            self.cantidad_pagos_bonificacion = nuevo_valor;
        }

        #[ink(message)]
        pub fn registrar_nuevo_socio(&mut self, dni: u128, nombre: String, categoria:Categoria, actividad: Option<Actividad>) {
            match self.buscar_socio(dni) {
                Some(idx) => panic!("Ya existe un socio con el dni {dni}: ({:?})", self.socios[idx]),
                None => (),
            }
            let mut valor_pago = self.obtener_precio(categoria);
            let deporte = match categoria {
                Categoria::CategoriaA => {
                    Actividad::Todas
                }
                Categoria::CategoriaB => {
                     match actividad {
                        None | Some(Actividad::Todas) | Some(Actividad::SoloGimnasio) => {
                            panic!("Debe elegir un deporte para la categoría B")
                        },
                        Some(otra) => otra
                    }
                },
                Categoria::CategoriaC => {
                    Actividad::SoloGimnasio
                }
            };
            
            let mut socio = Socio {
                dni,
                nombre,
                categoria,
                deporte
            };

            let mut vencimiento: Fecha = self.obtener_fecha_actual();
            vencimiento.sumar_dias(10);
            // self.cantidad_pagos += 1;
            // let mut pagos_cliente: Vec<Vpagos> = Vec::new();
            // self.generar_pagos(&mut pagos_cliente, fecha);
            let pago_final: Pago = Pago {
                id_socio: self.pagos.len() as u64,
                dni_socio: dni,
                monto: self.obtener_precio(categoria),
                // categoria,
                // deporte,
                pagado: None,
                vencimiento
                // vector_pagos: pagos_cliente
            };
            self.pagos.push(pago_final);
            self.socios.push(socio);
        }

        // Busca un socio y retorna su id
        fn buscar_socio(&self, dni: u128) -> Option<usize> {
            for (idx, socio) in self.socios.iter().enumerate() {
                if socio.dni == dni {
                    return Some(idx);
                }
            }
            None
        }

        // Obtiene el último pago del socio dado (que va a ser pendiente)
        fn buscar_ultimo_pago(&self, id_socio: u64) -> usize {
            // let socio = self.socios.get(id_socio).expect("Id de socio inválido");
            // rev() para buscar el último
            for (i, pago) in self.pagos.iter().enumerate().rev() {
                if pago.id_socio == id_socio {
                    assert!(pago.pagado.is_none(), "Todo socio debe tener registrado el siguiente pago pendiente");
                    return i
                }
            }
            panic!("Id de socio inválido")
        }

        #[ink(message)]
        pub fn realizar_pago(&mut self, dni: u128, monto: u128) {
            let id_socio = match self.buscar_socio(dni) {
                None => {
                    panic!("No existe ningún socio con el dni {dni}")
                },
                Some(id) => id
            };
            let socio = self.socios.get(id_socio).unwrap();
            let id_pago = self.buscar_ultimo_pago(id_socio as u64);
            let monto = self.obtener_precio(socio.categoria);
            let fecha_actual = self.obtener_fecha_actual();
            let pago = self.pagos.get_mut(id_pago).unwrap();
            pago.pagado = Some (fecha_actual);
            // generar el siguiente pago
            let mut fecha_siguiente = pago.vencimiento.clone();
            
        }

        // pub fn generar_pagos(&self, vector_pagos:&mut Vec<Vpagos>, fecha: Fecha){
        //     let i = fecha.get_mes();
        //     let mut ind=1;
        //     for a in i..12{
        //         let dia = fecha.get_dia();
        //         let mes = fecha.get_mes() + ind;
        //         ind += 1;
        //         let año = fecha.get_año();
        //         let f = Fecha::new(dia, mes, año).unwrap();
        //         let id = vector_pagos.len() + 1;
        //         let pago : Vpagos= Vpagos {
        //             id_pago: id as u8,
        //             fecha_de_pago: None,
        //             fecha_vencimiento: f.clone()
        //         };
        //         vector_pagos.push(pago);
        //     }
        // }

        // agregar Option<dni>
        #[ink(message)]
        pub fn mostrar_pagos(&self)->Vec<Pago>{
            self.pagos.clone()
        }
        
        //self.env().block_timestamp();  obtener momento de tiempo en milisengundos desde 01/01/1970
        pub fn obtener_fecha_actual(&self) -> Fecha {
            let milisegundos_desde_epoch = self.env().block_timestamp();
            let dias_desde_epoch = milisegundos_desde_epoch / 1000 / 60 / 60 / 24;
            let mut fecha = Fecha::new(1, 1, 1970).unwrap();
            fecha.sumar_dias(dias_desde_epoch as i32);
            fecha
        }
    }


    #[derive(scale::Decode, scale::Encode, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Actividad {
        // Categoría C
        SoloGimnasio,
        // Categoría B (gimnasio + uno de estos deportes)
        Futbol,
        Basquet,
        Rugby,
        Hockey,
        Natacion,
        Tenis,
        Paddel,
        // Categoría A
        Todas
    }

    
    #[derive(scale::Decode, scale::Encode, Debug,Clone, Copy)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Categoria{
        CategoriaA,
        CategoriaB,
        CategoriaC
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Socio{
        dni:u128,
        nombre: String,
        categoria: Categoria,
        deporte: Actividad,
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Pago {
        id_socio: u64,
        dni_socio: u128,
        // categoria: Categoria,
        // deporte: Actividad,
        monto: u128,
        vencimiento: Fecha,
        // Contiene la fecha y el monto pagado
        pagado: Option<Fecha>,
        // vector_pagos: Vec<Vpagos>
    }


    // #[derive(scale::Decode, scale::Encode, Debug, Clone)]
    // #[cfg_attr(
    //     feature = "std",
    //     derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    // )]
    // pub struct Vpagos {
    //     id_pago:u8,
    //     fecha_de_pago:Option<Fecha>,
    //     fecha_vencimiento:Fecha,
    // }
        
    
}

