#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(unused)]
pub use self::trabajo_final::ClubRef;
mod fecha;
#[ink::contract]
pub mod trabajo_final {   
    
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
        // 65535 meses suena como un máximo razonable...
        cantidad_pagos_bonificacion: u16,
        // el máximo es 100, así que con u8 sobra
        porcentaje_bonificacion: u8,
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

        /// Sólo el dueño puede cambiar la política (para evitar que alguien la cierre accidentalmente antes de pasarse de dueño).
        #[ink(message)]
        pub fn set_politica_autorizacion(&mut self, usar_la_politica: bool) {
            assert!(self.soy_el_dueño(), "Sólo el dueño puede cambiar la política de autorización.");
            self.politica_autorizacion = usar_la_politica;
        }
        
        /// Estrictamente si sos el dueño actual retorna true, o false en caso contrario.
        #[ink(message)]
        pub fn soy_el_dueño(&self) -> bool {
            self.dueño == Self::env().caller()
        }

        /// Si estás autorizado (con la política abierta, todos están autorizados) retorna true, o false en caso contrario.
        #[ink(message)]
        pub fn estoy_autorizado(&self) -> bool {
            !self.politica_autorizacion || self.soy_el_dueño() || self.autorizados.contains(&Self::env().caller())
        }

        /// Cambia el AccountId del dueño actual por el nuevo ingresado por parametro.
        #[ink(message)]
        pub fn cambiar_dueño(&mut self, nuevo_dueño: AccountId) {
            assert!(!self.politica_autorizacion || self.soy_el_dueño(), "No autorizado (no es dueño)");
            self.dueño = nuevo_dueño;
        }
        
        /// Retorna el AccountId del dueño actual.
        #[ink(message)]
        pub fn get_dueño(&self) -> AccountId {
            self.dueño
        }

        /// Agrega el AccountId ingresado como parametro al listado de autorizados.
        #[ink(message)]
        pub fn agregar_autorizado(&mut self, quien: AccountId) {
            assert!(!self.politica_autorizacion || self.soy_el_dueño(), "No autorizado (no es dueño)");
            assert!(!self.autorizados.contains(&quien), "Esa cuenta ya está autorizada");
            self.autorizados.push(quien);
        }

        /// Elimina el AccountId ingresado como parametro al listado de autorizados.
        #[ink(message)]
        pub fn quitar_autorizado(&mut self, quien: AccountId) {
            assert!(!self.politica_autorizacion || self.soy_el_dueño(), "No autorizado (no es dueño)");
            let i = self.autorizados.iter().position(|&cuenta| cuenta == quien);
            self.autorizados.swap_remove(i.expect("No se encuentra la cuenta autorizada."));
        }

        /// Retorna un vector [Vec] con los AccountId autorizados para operar el contrato.
        #[ink(message)]
        pub fn get_autorizados(&self) -> Vec<AccountId> {
            self.autorizados.clone()
        }

        /// Cambia el nombre del Club por el nombre ingresado por parametro.
        #[ink(message)]
        pub fn cambiar_nombre(&mut self, nuevo_nombre: String) {
            assert!(self.estoy_autorizado(), "No autorizado");
            self.nombre = nuevo_nombre;
        }

        /// Retorna el nombre actual del Club.
        #[ink(message)]
        pub fn get_nombre(&self) -> String {
            self.nombre.clone()
        }

        /// Establece el valor del precio de la categoria dada.
        #[ink(message)]
        pub fn set_precio(&mut self, categoria: Categoria, nuevo_valor: u128) {
            self._set_precio(categoria, nuevo_valor);
        }
        pub fn _set_precio(&mut self, categoria: Categoria, nuevo_valor: u128) {
            assert!(self.estoy_autorizado(), "No autorizado");
            self.precios[categoria.num()] = nuevo_valor;
        }

        /// Retorna el precio de la categoria dada.
        #[ink(message)]
        pub fn get_precio(&self, categoria: Categoria) -> u128 {
            self._get_precio(categoria)
        }
        fn _get_precio(&self, categoria: Categoria) -> u128 {
            self.precios[categoria.num()]
        }

        /// Establece la cantidad de pagos consecutivos necesarios para acceder a la bonificación de precio. 
        #[ink(message)]
        pub fn set_cantidad_pagos_bonificacion(&mut self, nuevo_valor:u16) {
            assert!(self.estoy_autorizado(), "No autorizado");
            assert!(nuevo_valor > 0);
            self.cantidad_pagos_bonificacion = nuevo_valor;
        }

        /// Retorna la cantidad de pagos consecutivos necesarios para acceder a la bonificación de precio.
        #[ink(message)]
        pub fn get_cantidad_pagos_bonificacion(&self) -> u16 {
            self.cantidad_pagos_bonificacion
        }

        /// Establece el porcentaje de bonificación de descuento por pagos consecutivos.
        #[ink(message)]
        pub fn set_porcentaje_bonificacion_pagos_consecutivos(&mut self, nuevo_valor:u8) {
            assert!(self.estoy_autorizado(), "No autorizado");
            assert!(nuevo_valor > 0 && nuevo_valor < 100);
            self.porcentaje_bonificacion = nuevo_valor;
        }

        /// Retorna el porcentaje de bonificación de descuento por pagos consecutivos.
        #[ink(message)]
        pub fn get_porcentaje_bonificacion_pagos_consecutivos(&self) -> u8 {
            self.porcentaje_bonificacion
        }

        /// Retorna un [Vec] con todos los [Socio]s registrados.
        #[ink(message)]
        pub fn get_socios(&self) -> Vec<Socio> {
            self.socios.clone()
        }
        
        /// Obtiene en un [Vec] todos los pagos de todos los socios o el socio con dni especificado en el Option.
        #[ink(message)]
        pub fn get_pagos(&self, socio: Option<u128>)-> Vec<Pago>{
            if let Some(dni) = socio {
                let id = self.buscar_socio(dni).expect("No existe socio con dni {dni}") as u64;
                self.pagos.iter().filter(|&p| p.id_socio == id).map(|p| p.clone()).collect()
            } else {
                self.pagos.clone()
            }
        }

        /// Registra un nuevo socio y genera el proximo pago con vencimiento en los proximos 10 dias.
        #[ink(message)]
        pub fn registrar_nuevo_socio(&mut self, dni: u128, nombre: String, categoria:Categoria) {
            self._registrar_nuevo_socio(dni, nombre, categoria);
        }
        fn _registrar_nuevo_socio(&mut self, dni: u128, nombre: String, categoria: Categoria) {
            assert!(self.estoy_autorizado(), "No autorizado");
            match self.buscar_socio(dni) {
                Some(idx) => panic!("Ya existe un socio con el dni {dni}: ({:?})", self.socios[idx]),
                None => (),
            }
            categoria.assert_valida();
            let mut valor_pago = self.get_precio(categoria);
            
            let mut socio = Socio {
                dni,
                nombre,
                categoria,
                pagos_a_tiempo_consecutivos: 0,
            };

            let mut vencimiento: Fecha = self.obtener_fecha_actual();
            vencimiento.sumar_dias(10);
            let pago_final: Pago = Pago {
                id_socio: self.pagos.len() as u64,
                monto: self.get_precio(categoria),
                pagado: None,
                vencimiento,
                es_descuento: false
            };
            self.pagos.push(pago_final);
            self.socios.push(socio);
        }

        /// Busca un socio y retorna un Option con su id en caso de existir en el registro, caso contrario
        /// retorna None.
        fn buscar_socio(&self, dni: u128) -> Option<usize> {
            for (idx, socio) in self.socios.iter().enumerate() {
                if socio.dni == dni {
                    return Some(idx);
                }
            }
            None
        }

        /// Devuelve el socio con la id dada.
        #[ink(message)]
        pub fn get_socio(&self, id: u64) -> Option<Socio> {
            self.socios.get(id as usize).cloned()
        }

        /// Obtiene id del último pago pendiente del socio dado.
        fn buscar_ultimo_pago(&self, id_socio: u64) -> usize {
            // rev() para buscar el último
            for (i, pago) in self.pagos.iter().enumerate().rev() {
                if pago.id_socio == id_socio {
                    assert!(pago.pagado.is_none(), "Todo socio debe tener registrado el siguiente pago pendiente");
                    return i
                }
            }
            panic!("Id de socio inválido")
        }

        /// Se registra el pago del dni ingresado solo si el monto ingresado es igual al monto a pagar
        /// según su pago pendiente.
        /// 
        /// Una vez registrado el pago actual se genera automaticamente el siguiente pago del usuario con su respectivo vencimiento
        /// y bonificación.
        #[ink(message)]
        pub fn realizar_pago(&mut self, dni: u128, monto: u128) {
            self._realizar_pago(dni, monto)
        }
        fn _realizar_pago(&mut self, dni: u128, monto: u128) {
            assert!(self.estoy_autorizado(), "No autorizado");
            let id_socio = match self.buscar_socio(dni) {
                None => {
                    panic!("No existe ningún socio con el dni {dni}")
                },
                Some(id) => id
            };
            let id_pago = self.buscar_ultimo_pago(id_socio as u64);
            let fecha_actual = self.obtener_fecha_actual();
            
            let pago = self.pagos.get_mut(id_pago).unwrap();
            assert_eq!(pago.monto, monto, "El monto a pagar es {}", pago.monto);
            assert!(!pago.es_pagado());
            pago.pagado = Some (fecha_actual);
            // los pagos con descuento no cuentan para el siguiente descuento
            if pago.es_pagado_a_tiempo().unwrap() && !pago.es_descuento {
                self.socios[id_socio].pagos_a_tiempo_consecutivos += 1;
            }

            // Generar el siguiente pago
            let mut nuevo_pago = pago.clone();

            let mut fecha_siguiente = pago.vencimiento.clone();
            fecha_siguiente.sumar_dias(30);
            nuevo_pago.pagado = None;
            nuevo_pago.vencimiento = fecha_siguiente;
            nuevo_pago.es_descuento = false;
            let socio = &self.socios[id_socio];
            nuevo_pago.monto = self.get_precio(socio.categoria);

            // Aplicar bonificación
            if socio.pagos_a_tiempo_consecutivos >= self.cantidad_pagos_bonificacion {
                let procentaje_del_total = (100 - self.porcentaje_bonificacion) as u128;
                self.socios[id_socio].pagos_a_tiempo_consecutivos = 0;
                nuevo_pago.es_descuento = true;
                assert!(nuevo_pago.monto <= u128::MAX / procentaje_del_total, "valor demasiado grande para aplicar descuento");
                nuevo_pago.monto = nuevo_pago.monto * procentaje_del_total / 100; 
            }

            self.pagos.push(nuevo_pago);
        }

        /// Retorna la fecha actual en un Struct con año, mes y día.
        #[ink(message)]
        pub fn obtener_fecha_actual(&self) -> Fecha {
            self._obtener_fecha_actual()
        }

        // self.env().block_timestamp(): tiempo en milisengundos desde 01/01/1970
        fn _obtener_fecha_actual(&self) -> Fecha {
            let milisegundos_desde_epoch = self.env().block_timestamp();
            let dias_desde_epoch = milisegundos_desde_epoch / 1000 / 60 / 60 / 24;
            let mut fecha = Fecha::new(1, 1, 1970).unwrap();
            fecha.sumar_dias(dias_desde_epoch as i32);
            fecha
        }
    }


    #[derive(scale::Decode, scale::Encode, Debug, Clone, Copy, PartialEq, Eq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Actividad {
        // Categoría C
        Gimnasio,
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

    
    #[derive(scale::Decode, scale::Encode, Debug, Clone, Copy, PartialEq, Eq)]
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
        /// Retorna el Índice de la categoría (0, 1 o 2 para A, B y C respectivamente) 
        pub fn num(&self) -> usize {
            use Categoria::*;
            match self {
                CategoriaA => 0,
                CategoriaB(_) => 1,
                CategoriaC => 2,
            }
        }
        /// Retorna true si un socio de la categoría puede acceder a la actividad dada.
        pub fn puede_acceder_a(&self, actividad: Actividad) -> bool {
            use Categoria::*;
            match self {
                CategoriaA => true,
                CategoriaB(act) => actividad == Actividad::Gimnasio || actividad == *act,
                CategoriaC => actividad == Actividad::Gimnasio
            }
        }
        /// Causa un panic si la infomarción de la categoría es inválida
        pub fn assert_valida(&self) {
            use Categoria::*;
            assert_ne!(self, &CategoriaB(Actividad::Gimnasio), "El gimnasio está disponible para todos los socios; no corresponde a la elección en la categoría B.");
        }
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone, PartialEq, Eq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Socio{
        dni:u128,
        nombre: String,
        categoria: Categoria,
        pagos_a_tiempo_consecutivos: u16,
    }
    impl Socio {
        /// Retorna el DNI del socio.
        pub fn get_dni(&self) -> u128 {
            self.dni
        }
        /// Retorna el Nombre del socio.
        pub fn get_nombre(&self) -> &str {
            &self.nombre
        }
        /// Retorna la categoria seleccionada por el Socio.
        pub fn get_categoria(&self) -> Categoria {
            self.categoria
        }
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone, PartialEq, Eq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Pago {
        id_socio: u64,
        monto: u128,
        vencimiento: Fecha,
        pagado: Option<Fecha>,
        es_descuento: bool,
    }

    impl Pago {
        /// Retorna el monto a pagar.
        pub fn get_monto(&self) -> u128 {
            self.monto
        }

        /// Retorna id del socio.
        pub fn get_socio(&self) -> u64 {
            self.id_socio
        }

        /// Retorna la fecha de vencimiento del pago.
        pub fn get_vencimiento(&self) -> Fecha {
            self.vencimiento
        }

        /// Retorna un [Option] con la fecha de pago o None si aun no fue pagado.
        pub fn get_pagado(&self) -> Option<Fecha> {
            self.pagado
        }

        /// Retorna true si el pago es con descuento o false en caso contrario.
        pub fn get_es_descuento(&self) -> bool {
            self.es_descuento
        }

        /// Retorna true si fue pagado, o false en caso contrario.
        pub fn es_pagado(&self) -> bool {
            self.pagado.is_some()
        }

        /// Retorna un [Option] con true si se realizó el pago antes o en la fecha de vencimiento,
        /// false en caso contrario, y None si aún no se realizó.
        pub fn es_pagado_a_tiempo(&self) -> Option<bool> {
            // no es mayor = es menor o igual = se pagó en el día de vencimiento o antes
            self.pagado.map(|fecha_pagado| !fecha_pagado.es_mayor(&self.vencimiento))
        }

        /// Devuelve true si, a la fecha ingresada, el pago está pendiente y ya pasó la fecha de vencimiento.
        pub fn es_moroso(&self, fecha_actual: Fecha) -> bool {
            if self.es_pagado() {return false;}
            !fecha_actual.es_mayor(&self.vencimiento)
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
        assert_eq!(club.get_cantidad_pagos_bonificacion(), 5);
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
        assert_eq!(club.get_pagos(Some(1))[0].get_monto(), club.get_precio(CategoriaB(Tenis)));
        assert_eq!(club.get_pagos(Some(2))[0].get_monto(), club.get_precio(CategoriaC));
    }


    #[ink::test]
    fn bonificacion_test() {
        let mut club = generar_club();
        club.set_cantidad_pagos_bonificacion(2);
        club.set_porcentaje_bonificacion_pagos_consecutivos(25);
        club.set_precio(CategoriaA, 100);
        club.registrar_nuevo_socio(0, "".into(), Categoria::CategoriaA);
        club.realizar_pago(0, 100);
        club.realizar_pago(0, 100);
        club.realizar_pago(0, 75);
        club.realizar_pago(0, 100);
        club.realizar_pago(0, 100);
        club.realizar_pago(0, 75);
        club.realizar_pago(0, 100);
        club.realizar_pago(0, 100);
        club.realizar_pago(0, 75);
        let pagos = club.get_pagos(None);
        assert_eq!(pagos[0].get_monto(), 100);
        assert_eq!(pagos[1].get_monto(), 100);
        assert_eq!(pagos[2].get_monto(), 75);
        assert_eq!(pagos[3].get_monto(), 100);
        assert_eq!(pagos[4].get_monto(), 100);
        assert_eq!(pagos[5].get_monto(), 75);
    }

    #[ink::test]
    fn pagos_exactos_test() {
        let mut club = generar_club();
        club.set_cantidad_pagos_bonificacion(2);
        club.set_porcentaje_bonificacion_pagos_consecutivos(25);
        club.set_precio(CategoriaA, 100);
        club.registrar_nuevo_socio(0, "".into(), Categoria::CategoriaA);
        club.realizar_pago(0, 100);
        assert!(panic::catch_unwind(|| {
            club.clone().realizar_pago(0, 101);
        }).is_err());
        assert!(panic::catch_unwind(|| {
            club.clone().realizar_pago(0, 99);
        }).is_err());
        assert!(panic::catch_unwind(|| {
            club.clone().realizar_pago(0, 75);
        }).is_err());
        club.realizar_pago(0, 100);
        assert!(panic::catch_unwind(|| {
            club.clone().realizar_pago(0, 76);
        }).is_err());
        assert!(panic::catch_unwind(|| {
            club.clone().realizar_pago(0, 74);
        }).is_err());
        assert!(panic::catch_unwind(|| {
            club.clone().realizar_pago(0, 100);
        }).is_err());
        club.realizar_pago(0, 75);
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
        // Bob no puede agregar autorizados
        assert!(panic::catch_unwind(|| {
            club.clone().agregar_autorizado(bob());
        }).is_err());

        ser_alicia();
        club.agregar_autorizado(bob());
        club.agregar_autorizado(carlos());
        // doble autorización no es posible
        assert!(panic::catch_unwind(|| {
            club.clone().agregar_autorizado(bob());
        }).is_err());
        ser_bob();
        assert!(club.estoy_autorizado());
        ser_alicia();
        club.quitar_autorizado(bob());
        ser_bob();
        assert!(!club.estoy_autorizado());
        ser_carlos();
        assert!(club.estoy_autorizado());
        // los autorizados no pueden agregar más autorizados
        assert!(panic::catch_unwind(|| {
            club.clone().agregar_autorizado(bob());
        }).is_err());
        assert!(panic::catch_unwind(|| {
            club.clone().cambiar_dueño(carlos());
        }).is_err());
        ser_alicia();
        club.cambiar_dueño(carlos());
        assert!(!club.soy_el_dueño());
        assert!(!club.estoy_autorizado());
        ser_carlos();
        assert!(club.soy_el_dueño());
    }

    #[ink::test]
    fn no_autorizacion_test() {
        // parecido al anterior, pero siempre están autorizados
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
    fn autorizacion_leer_cosas_test() {
        let mut club = generar_club();
        ser_alicia();
        club.set_politica_autorizacion(true);
        club.agregar_autorizado(carlos());
        club.registrar_nuevo_socio(0, "Alicia".to_string(), CategoriaC);
        club.realizar_pago(0, 2000);
        club.realizar_pago(0, 2000);
        club.realizar_pago(0, 2000);
        ser_bob();
        // Incluso sin autorización, todas estas cosas se deberían poder leer
        assert_eq!(club.get_autorizados(), vec![carlos()]);
        assert_eq!(club.get_cantidad_pagos_bonificacion(), 5);
        assert_eq!(club.get_porcentaje_bonificacion_pagos_consecutivos(), 10);
        assert_eq!(club.get_dueño(), alicia());
        assert_eq!(club.get_nombre(), "Seminario Rust");
        assert_eq!(club.get_precio(CategoriaA), 5000);
        assert_eq!(club.get_precio(CategoriaB(Paddle)), 3000);
        assert_eq!(club.get_precio(CategoriaC), 2000);
        assert_eq!(club.get_pagos(None).len(), 4);
    }

    #[ink::test]
    fn autorizacion_hacer_cosas() {
        let mut club = generar_club();
        ser_alicia();
        club.set_politica_autorizacion(true);
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
            club.clone().set_cantidad_pagos_bonificacion(1);
        }).is_err());
        assert!(panic::catch_unwind(|| {
            club.clone().set_porcentaje_bonificacion_pagos_consecutivos(100);
        }).is_err());
        assert!(panic::catch_unwind(|| {
            club.clone().set_politica_autorizacion(false);
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
