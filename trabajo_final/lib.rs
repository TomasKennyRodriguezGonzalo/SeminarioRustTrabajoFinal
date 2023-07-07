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
    #[derive(Clone)]
    pub struct Club {
        nombre: String,
        pagos: Vec<Pago>,
        socios: Vec<Socio>,
        // Precio de cada categoría, en tokens por mes
        precios: [u128; 3],
        // cantidad_pagos: u128,
        cantidad_pagos_bonificacion: u128,
        porcentaje_bonificacion: u128,
        // permisos, etc.
        politica_autorizacion: bool,
        dueño: AccountId,
        autorizados: Vec<AccountId>,
    }

    impl Club {
        #[ink(constructor)]
        pub fn new(dueño: AccountId) -> Self {
            Self::_new(dueño)
        }
        fn _new(dueño: AccountId) -> Self {
            Self { 
                nombre: "Seminario Rust".into(),
                pagos : Vec::new(),
                socios: Vec::new(),
                precios: [5000, 3000, 2000],
                cantidad_pagos_bonificacion:5,
                porcentaje_bonificacion: 10,
                politica_autorizacion: true,
                dueño,
                autorizados: Vec::new(),
            }
        }

        #[ink(message)]
        pub fn set_politica_autorizacion(&mut self, usar_la_politica: bool) {
            self.politica_autorizacion = usar_la_politica;
        }
        #[ink(message)]
        pub fn cambiar_dueño(&mut self, nuevo_dueño: AccountId) {

        }
        #[ink(message)]
        pub fn get_dueño(&self) -> AccountId {
            self.dueño
        }

        #[ink(message)]
        pub fn agregar_autorizado(&mut self, quien: AccountId) {
            
        }
        #[ink(message)]
        pub fn quitar_autorizado(&mut self, quien: AccountId) {
            
        }
        
        #[ink(message)]
        pub fn get_autorizados(&self) -> Vec<AccountId> {
            self.autorizados.clone()
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
        pub fn soy_el_dueño(&self) -> bool {
            self.dueño == Self::env().caller()
        }

        #[ink(message)]
        pub fn estoy_autorizado(&self) -> bool {
            self.soy_el_dueño() || self.autorizados.contains(&Self::env().caller())
        }

        #[ink(message)]
        pub fn set_precio(&mut self, categoria: Categoria, nuevo_valor: u128) {
            self._set_precio(categoria, nuevo_valor);
        }
        pub fn _set_precio(&mut self, categoria: Categoria, nuevo_valor: u128) {
            self.precios[categoria.num()] = nuevo_valor;
        }

        #[ink(message)]
        pub fn get_precio(&self, categoria: Categoria) -> u128 {
            self._get_precio(categoria)
        }
        fn _get_precio(&self, categoria: Categoria) -> u128 {
            self.precios[categoria.num()]
        }

        #[ink(message)]
        pub fn set_bonificacion_pagos_consecutivos(&mut self, nuevo_valor:u128) {
            self.cantidad_pagos_bonificacion = nuevo_valor;
        }
        #[ink(message)]
        pub fn get_bonificacion_pagos_consecutivos(&self) -> u128 {
            self.cantidad_pagos_bonificacion
        }

        #[ink(message)]
        pub fn set_porcentaje_bonificacion_pagos_consecutivos(&mut self, nuevo_valor:u128) {
            self.porcentaje_bonificacion = nuevo_valor;
        }
        #[ink(message)]
        pub fn get_porcentaje_bonificacion_pagos_consecutivos(&self) -> u128 {
            self.porcentaje_bonificacion
        }

        #[ink(message)]
        pub fn registrar_nuevo_socio(&mut self, dni: u128, nombre: String, categoria:Categoria) {
            self._registrar_nuevo_socio(dni, nombre, categoria);
        }
        fn _registrar_nuevo_socio(&mut self, dni: u128, nombre: String, categoria: Categoria) {
            match self.buscar_socio(dni) {
                Some(idx) => panic!("Ya existe un socio con el dni {dni}: ({:?})", self.socios[idx]),
                None => (),
            }
            let mut valor_pago = self.get_precio(categoria);
            
            let mut socio = Socio {
                dni,
                nombre,
                categoria,
            };

            let mut vencimiento: Fecha = self.obtener_fecha_actual();
            vencimiento.sumar_dias(10);
            // self.cantidad_pagos += 1;
            // let mut pagos_cliente: Vec<Vpagos> = Vec::new();
            // self.generar_pagos(&mut pagos_cliente, fecha);
            let pago_final: Pago = Pago {
                id_socio: self.pagos.len() as u64,
                dni_socio: dni,
                monto: self.get_precio(categoria),
                // categoria,
                // deporte,
                pagado: None,
                vencimiento,
                es_descuento: false
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
            let monto = self.get_precio(socio.categoria);
            let fecha_actual = self.obtener_fecha_actual();
            let pago = self.pagos.get_mut(id_pago).unwrap();
            pago.pagado = Some (fecha_actual);
            // generar el siguiente pago (SIN TERMINAR)
            let mut fecha_siguiente = pago.vencimiento.clone();
            fecha_siguiente.sumar_dias(30);

        }

        // cliente: Option<dni>
        #[ink(message)]
        pub fn get_pagos(&self, cliente: Option<u128>)->Vec<Pago>{
            self.pagos.clone()
        }
        
        // self.env().block_timestamp(): tiempo en milisengundos desde 01/01/1970
        fn obtener_fecha_actual(&self) -> Fecha {
            let milisegundos_desde_epoch = self.env().block_timestamp();
            let dias_desde_epoch = milisegundos_desde_epoch / 1000 / 60 / 60 / 24;
            let mut fecha = Fecha::new(1, 1, 1970).unwrap();
            fecha.sumar_dias(dias_desde_epoch as i32);
            fecha
        }
    }


    #[derive(scale::Decode, scale::Encode, Debug, Clone, Copy)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Actividad {
        // Categoría C
        // SoloGimnasio,
        // Categoría B (gimnasio + uno de estos deportes)
        Futbol,
        Basquet,
        Rugby,
        Hockey,
        Natacion,
        Tenis,
        Paddle,
        // Categoría A
        // Todas
    }

    
    #[derive(scale::Decode, scale::Encode, Debug, Clone, Copy)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Categoria{
        CategoriaA,
        CategoriaB(Actividad),
        CategoriaC
    }

    impl Categoria {
        pub fn num(&self) -> usize {
            use Categoria::*;
            match self {
                CategoriaA => 0,
                CategoriaB(_) => 1,
                CategoriaC => 2,
            }
        }
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
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Pago {
        id_socio: u64,
        dni_socio: u128,
        monto: u128,
        vencimiento: Fecha,
        pagado: Option<Fecha>,
        es_descuento: bool,
    }

    impl Pago {
        pub fn get_monto(&self) -> u128 {
            self.monto
        }
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use crate::trabajo_final::*;
    use Categoria::*;
    use Actividad::*;
    use ink::codegen::{StaticEnv, Env};
    use ink_env::{DefaultEnvironment, Environment};

    type TipoCuenta = <DefaultEnvironment as Environment>::AccountId;
    fn cuentas() -> ink_env::test::DefaultAccounts<DefaultEnvironment> {
        ink_env::test::default_accounts::<DefaultEnvironment>()
    }
    fn set_cuenta(quien: TipoCuenta) {
        ink_env::test::set_caller::<DefaultEnvironment>(quien);
    }

    fn alicia() -> TipoCuenta {cuentas().alice}
    fn bob() -> TipoCuenta {cuentas().bob}
    fn carlos() -> TipoCuenta {cuentas().charlie}
    fn dilan() -> TipoCuenta {cuentas().django}
    fn eva() -> TipoCuenta {cuentas().eve}
    fn franco() -> TipoCuenta {cuentas().frank}
        
    fn ser_alicia() {set_cuenta(alicia())}
    fn ser_bob() {set_cuenta(bob())}
    fn ser_carlos() {set_cuenta(carlos())}
    fn ser_dilan() {set_cuenta(dilan())}
    fn ser_eva() {set_cuenta(eva())}
    fn ser_franco() {set_cuenta(franco())}

    fn generar_club() -> Club {
        let mut club = Club::new(alicia());
        club.set_politica_autorizacion(false);
        club
    }

    #[ink::test]
    fn valores_default_test() {
        let club = generar_club();
        assert_eq!(club.get_precio(CategoriaA), 5000);
        assert_eq!(club.get_precio(CategoriaB(Tenis)), 3000);
        assert_eq!(club.get_precio(CategoriaC), 2000);
        assert_eq!(club.get_nombre(), "Seminario Rust");
        assert_eq!(club.get_pagos(None).len(), 0);
        assert_eq!(club.get_bonificacion_pagos_consecutivos(), 5);
        assert_eq!(club.get_porcentaje_bonificacion_pagos_consecutivos(), 10);
    }

    #[ink::test]
    fn registrar_socio_test() {
        let mut club = generar_club();
        club.registrar_nuevo_socio(0, "".into(), Categoria::CategoriaA);
    }

    #[ink::test]
    #[should_panic]
    fn socio_inexistente_test() {
        let mut club = generar_club();
        club.registrar_nuevo_socio(0, "".into(), Categoria::CategoriaA);
        club.realizar_pago(1, 100000);
    }

    #[should_panic]
    #[ink::test]
    fn registrar_socio_repetido_test() {
        let mut club = generar_club();
        club.registrar_nuevo_socio(0, "".into(), Categoria::CategoriaA);
        club.registrar_nuevo_socio(0, "".into(), Categoria::CategoriaA);
    }

    #[ink::test]
    fn realizar_pagos_test() {
        let mut club = generar_club();
        club.registrar_nuevo_socio(5, "".into(), Categoria::CategoriaA);
        // Error: no existe cliente
        let res = panic::catch_unwind(|| {
            club.clone().realizar_pago(4, u128::MAX);
        });
        assert!(res.is_err());
        // Error: monto insuficiente
        let res = panic::catch_unwind(|| {
            club.clone().realizar_pago(5, club.get_precio(CategoriaA) - 1);
        });
        assert!(res.is_err());
        // Funciona
        club.realizar_pago(5, club.get_precio(CategoriaA));
    }

    #[ink::test]
    fn obtener_pagos_test() {
        let mut club = generar_club();
        club.registrar_nuevo_socio(0, "".into(), Categoria::CategoriaA);
        club.registrar_nuevo_socio(1, "".into(), Categoria::CategoriaB(Tenis));
        club.registrar_nuevo_socio(2, "".into(), Categoria::CategoriaC);
        club.realizar_pago(2, club.get_precio(CategoriaC));
        assert_eq!(club.get_pagos(None).len(), 4);
        assert_eq!(club.get_pagos(Some(0)).len(), 1);
        assert_eq!(club.get_pagos(Some(1)).len(), 1);
        assert_eq!(club.get_pagos(Some(2)).len(), 2);
        assert_eq!(club.get_pagos(Some(0))[0].get_monto(), club.get_precio(CategoriaA));
        assert_eq!(club.get_pagos(Some(1))[0].get_monto(), club.get_precio(CategoriaA));
        assert_eq!(club.get_pagos(Some(2))[0].get_monto(), club.get_precio(CategoriaA));
    }

    #[ink::test]
    fn bonificacion_test() {
        let mut club = generar_club();
        club.set_bonificacion_pagos_consecutivos(2);
        club.set_porcentaje_bonificacion_pagos_consecutivos(25);
        club.set_precio(CategoriaA, 100);
        club.registrar_nuevo_socio(0, "".into(), Categoria::CategoriaA);
        for _ in 0..10 {
            club.realizar_pago(0, 9999); // Qué hacer si el monto es mayor?
        }
        assert_eq!(club.get_pagos(None)[0].get_monto(), 100);
        assert_eq!(club.get_pagos(None)[1].get_monto(), 100);
        assert_eq!(club.get_pagos(None)[2].get_monto(), 75);

        assert_eq!(club.get_pagos(None)[3].get_monto(), 100);
        assert_eq!(club.get_pagos(None)[4].get_monto(), 100);
        assert_eq!(club.get_pagos(None)[5].get_monto(), 75);
    }

    #[ink::test]
    fn autorizacion_test() {
        let mut club = generar_club();
        club.set_politica_autorizacion(true);
        ser_alicia();
        assert!(club.soy_el_dueño());
        assert!(club.estoy_autorizado());
        ser_bob();
        assert!(!club.soy_el_dueño());
        assert!(!club.estoy_autorizado());
        club.agregar_autorizado(bob());
        club.agregar_autorizado(carlos());
        assert!(club.estoy_autorizado());
        club.quitar_autorizado(bob());
        assert!(!club.estoy_autorizado());
        ser_carlos();
        assert!(club.estoy_autorizado());
    }

    #[ink::test]
    fn no_autorizacion_test() {
        // Igual que el anterior, pero siempre están autorizados
        let mut club = generar_club();
        club.set_politica_autorizacion(false);
        ser_alicia();
        assert!(club.soy_el_dueño());
        assert!(club.estoy_autorizado());
        ser_bob();
        assert!(!club.soy_el_dueño());
        assert!(club.estoy_autorizado());
        club.agregar_autorizado(bob());
        club.agregar_autorizado(carlos());
        assert!(club.estoy_autorizado());
        club.quitar_autorizado(bob());
        assert!(club.estoy_autorizado());
        ser_carlos();
        assert!(club.estoy_autorizado());
    }


    #[ink::test]
    fn autorizacion_leer_cosas() {
        let mut club = generar_club();
        ser_alicia();
        club.agregar_autorizado(carlos());
        club.registrar_nuevo_socio(0, "Alicia".to_string(), CategoriaC);
        club.realizar_pago(0, 10000);
        club.realizar_pago(0, 10000);
        club.realizar_pago(0, 10000);
        ser_bob();
        // Incluso sin autorización, todas estas cosas se deberían poder leer
        assert_eq!(club.get_autorizados(), vec![carlos()]);
        assert_eq!(club.get_bonificacion_pagos_consecutivos(), 5);
        assert_eq!(club.get_porcentaje_bonificacion_pagos_consecutivos(), 10);
        assert_eq!(club.get_dueño(), alicia());
        assert_eq!(club.get_nombre(), "Seminario Rust");
        assert_eq!(club.get_precio(CategoriaA), 5000);
        assert_eq!(club.get_precio(CategoriaB(Paddle)), 3000);
        assert_eq!(club.get_precio(CategoriaC), 2000);
        assert_eq!(club.get_pagos(None).len(), 3);
    }

    #[ink::test]
    fn autorizacion_hacer_cosas() {
        let mut club = generar_club();
        ser_alicia();
        club.registrar_nuevo_socio(0, "Alicia".to_string(), CategoriaC);
        ser_bob();
        // bob no debería poder hacer nada
        assert!(panic::catch_unwind(|| {
            club.clone().cambiar_dueño(bob());
        }).is_err());
        assert!(panic::catch_unwind(|| {
            club.clone().agregar_autorizado(bob());
        }).is_err());
        assert!(panic::catch_unwind(|| {
            club.clone().set_bonificacion_pagos_consecutivos(1);
        }).is_err());
        assert!(panic::catch_unwind(|| {
            club.clone().set_porcentaje_bonificacion_pagos_consecutivos(100);
        }).is_err());
        assert!(panic::catch_unwind(|| {
            club.clone().set_politica_autorizacion(true);
        }).is_err());
        assert!(panic::catch_unwind(|| {
            club.clone().set_precio(CategoriaA, 0);
        }).is_err());
        assert!(panic::catch_unwind(|| {
            club.clone().set_precio(CategoriaB(Futbol), 0);
        }).is_err());
        assert!(panic::catch_unwind(|| {
            club.clone().set_precio(CategoriaC, 0);
        }).is_err());
        assert!(panic::catch_unwind(|| {
            club.clone().registrar_nuevo_socio(1, "Bob".to_string(), CategoriaA);
        }).is_err());
        assert!(panic::catch_unwind(|| {
            club.clone().realizar_pago(0, u128::MAX);
        }).is_err());
    }

    #[ink::test]
    fn autorizacion_con_cosas_test() {
        let mut club = generar_club();
        club.set_politica_autorizacion(true);
    }
    
}