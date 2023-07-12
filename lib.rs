#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
mod trabajo_final_reporte {
    use ink::prelude::string::String;
    use ink::prelude::collections::HashSet;
    use trabajo_final::ClubRef;
    use trabajo_final::trabajo_final::{Socio, Categoria::*};

    #[ink(storage)]
    pub struct TrabajoFinalReporte {
        club: ClubRef,
    }

    impl TrabajoFinalReporte {
        #[ink(constructor)]
        pub fn new(club: ClubRef) -> Self {
            Self { club }
        }

        /// Se puede cambiar el club del que se hacen los reportes
        #[ink(message)]
        pub fn cambiar_club(&mut self, nuevo_club: ClubRef) {
            self.club = nuevo_club;
        }

        /// Test simple para ver que funcione la comunicación con el contrato
        #[ink(message)]
        pub fn obtener_nombre(&self) -> String {
            self.club.get_nombre()
        }

        #[ink(message)]
        pub fn obtener_socios_morosos(&self) -> Vec<Socio> {
            // socios guardados por id
            let mut socios_morosos: HashSet<u64> = HashSet::new();
            let pagos = self.club.get_pagos(None);
            let fecha_actual = self.club.obtener_fecha_actual();
            for pago in pagos {
                if pago.es_moroso(fecha_actual) {
                    socios_morosos.insert(pago.get_socio());
                }
            }
            socios_morosos.iter().map(|&id| self.club.get_socio(id).unwrap()).collect()
        }

        // Recaudación durante el mes pedido
        #[ink(message)]
        pub fn informe_recaudacion(&self, año: i32, mes: i8) -> Vec<(String, u128)> {
            let pagos = self.club.get_pagos(None);
            let mut cantidades = [0; 3];
            for pago in pagos {
                if let Some(fecha_pagado) = pago.get_pagado() {
                    if fecha_pagado.get_año() == año && fecha_pagado.get_mes() == mes {
                        let i = match self.club.get_socio(pago.get_socio()).unwrap().get_categoria() {
                            CategoriaA => {0},
                            CategoriaB(_) => {1},
                            CategoriaC => {2},
                        };
                        cantidades[i] += pago.get_monto();
                    }
                }
            }
            vec![
                ("Categoría A".into(), cantidades[0]),
                ("Categoría B".into(), cantidades[1]),
                ("Categoría C".into(), cantidades[2]),
            ]
        }
    }
}
