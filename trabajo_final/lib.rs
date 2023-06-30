#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(unused)]
pub use self::trabajo_final::ClubRef;
mod fecha;
#[ink::contract]
mod trabajo_final {   
    
    use crate::fecha::Fecha;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    #[derive(scale::Decode, scale::Encode, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Actividad{
        SoloGimnasio,
        Futbol,
        Basquet,
        Rugby,
        Hockey,
        Natacion,
        Tenis,
        Paddel,
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

    }
    #[derive(scale::Decode, scale::Encode, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Pago{
        id_socio:u8,
        socio:Socio,
        categoria:Categoria,
        monto:u128,
        deporte:Actividad,
        vector_pagos:Vec<Vpagos>
    }
    #[derive(scale::Decode, scale::Encode, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Vpagos{
        id_pago:u8,
        fecha_de_pago:Option<Fecha>,
        fecha_vencimiento:Fecha,
    }
        
    #[ink(storage)]    
    pub struct Club {
        nombre: String,
        pagos: Vec<Pago>,
        cat_a:u128,
        cat_b:u128,
        cat_c:u128,
        cantidad_pagos:u128,
        cantidad_pagos_bonificacion:u128,
        porcentaje_bonificacion:u128
    }

    impl Club {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                nombre: "Seminario Rust".into(),
                pagos : Vec::new(),
                cat_a: 5000,
                cat_b: 3000,
                cat_c: 2000,
                cantidad_pagos: 0,
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
        pub fn set_precio_cat_a(&mut self,nuevo_valor:u128) {
            self.cat_a = nuevo_valor;
        }

        #[ink(message)]
        pub fn set_precio_cat_b(&mut self,nuevo_valor:u128) {
            self.cat_b = nuevo_valor;
        }

        #[ink(message)]
        pub fn set_precio_cat_c(&mut self,nuevo_valor:u128) {
            self.cat_c = nuevo_valor;
        }
        #[ink(message)]
        pub fn set_bonificacion_pagos_consecutivos(&mut self,nuevo_valor:u128) {
            self.cantidad_pagos_bonificacion = nuevo_valor;
        }

        #[ink(message)]
        pub fn set_porcentaje_bonificacion_pagos_consecutivos(&mut self,nuevo_valor:u128) {
            self.cantidad_pagos_bonificacion = nuevo_valor;
        }

        #[ink(message)]
        pub fn  registrar_nuevo_socio(&mut self,dni: u128, categoria:Categoria, actividad : Option<Actividad>){
            let mut socio= Socio{dni};
            let categoria = categoria.clone();
            let mut valor_pago ;
            let mut deporte:Actividad;
            match categoria {
                Categoria::CategoriaA => {valor_pago = self.cat_a; deporte = Actividad::Todas}
                Categoria::CategoriaB => {valor_pago = self.cat_b; deporte = actividad.clone().unwrap();},
                Categoria::CategoriaC => {valor_pago = self.cat_c; deporte = Actividad::SoloGimnasio}
            }
            let fecha:Fecha = self.obtener_fecha();
            self.cantidad_pagos+=1;
            let mut pagos_cliente:Vec<Vpagos>=Vec::new();
            self.generar_pagos(& mut pagos_cliente, fecha);
            let pago_final:Pago = Pago { id_socio:self.cantidad_pagos as u8, socio, categoria, monto:valor_pago, deporte, vector_pagos:pagos_cliente.clone() };
            self.pagos.push(pago_final);
        }


        pub fn generar_pagos(&self, vector_pagos:&mut Vec<Vpagos>, fecha: Fecha){
            let i = fecha.get_mes();
            let mut ind=1;
            for a in i..12{
                let dia = fecha.get_dia();
                let mes = fecha.get_mes()+ind;
                ind+=1;
                let año = fecha.get_año();
                let f = Fecha::new(dia, mes, año).unwrap();
                let id = vector_pagos.len()+1;
                let pago : Vpagos= Vpagos { id_pago: id as u8, fecha_de_pago: None, fecha_vencimiento: f.clone() };
                vector_pagos.push(pago);
            }
        }

        #[ink(message)]
        pub fn mostrar_pagos(&self)->Vec<Pago>{
            self.pagos.clone()
        }
        
        //self.env().block_timestamp();  obtener momento de tiempo en milisengundos desde 01/01/1970
        pub fn obtener_fecha(&self)->Fecha {
            let milisegundos_desde_epoch = self.env().block_timestamp();
            let dias_desde_epoch = milisegundos_desde_epoch / 1000 / 60 / 60 / 24;
            let mut fecha = Fecha::new(1, 1, 1970).unwrap();
            fecha.sumar_dias(dias_desde_epoch as i32);
            fecha
        }

    }
}

