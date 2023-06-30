#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
mod trabajo_final_reporte {
    use ink::prelude::string::String;
    use trabajo_final::ClubRef;

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
    }
}
