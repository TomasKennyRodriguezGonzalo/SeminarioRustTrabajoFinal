#[derive(scale::Decode, scale::Encode, Debug, Clone, Copy)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Fecha {
    dia: i8,
    mes: i8,
    año: i32,
}

impl Fecha {
    // No se permite construir fechas inválidas
    pub fn new(dia: i8, mes: i8, año: i32) -> Result<Fecha, ()> {
        let fecha_tentativa = Fecha {dia, mes, año};
        if fecha_tentativa.es_fecha_valida() {
            Ok(fecha_tentativa)
        } else {
            Err(())
        }
    }

    pub fn get_dia(&self) -> i8 {
        self.dia
    }
    pub fn get_mes(&self) -> i8 {
        self.mes
    }
    pub fn get_año(&self) -> i32 {
        self.año
    }

    fn es_fecha_valida(&self) -> bool {
        // todos los años son válidos
        self.mes >= 1 && self.mes <= 12 &&
        self.dia >= 1 && self.dia <= self.ultimo_dia_mes()
    }

    pub fn es_bisiesto(&self) -> bool {
        if self.año % 4 == 0 { // Tal vez bisiesto, a no ser...
            if self.año % 100 == 0 { // Ahora no, excepto que...
                if self.año % 400 == 0 {
                    return true
                }
                return false
            }
            return true
        }
        false
    }

    // Retorna el último día del mes actual.
    fn ultimo_dia_mes(&self) -> i8 {
        // 31, 2X, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31
        match self.mes {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => {
                31
            }
            4 | 6 | 9 | 11 => {
                30
            }
            2 => {
                if self.es_bisiesto() {29} else {28}
            }
            m => {
                panic!("El mes {m} es inválido!")
            }
        }
    }

    fn dias_hasta_fin_de_mes(&self) -> i8 {
        self.ultimo_dia_mes() - self.dia
    }

    pub fn sumar_dias(&mut self, mut dias: i32) {
        if dias < 0 { // No es la idea, pero no cuesta nada permitirlo
            self.restar_dias(-dias);
            return
        }
        while dias > self.dias_hasta_fin_de_mes() as i32 {
            // Vamos pasando los meses, parandonos en el primero del mes siguiente
            dias -= self.dias_hasta_fin_de_mes() as i32 + 1;
            self.dia = 1;
            self.mes += 1;
            if self.mes == 13 {
                self.mes = 1;
                self.año += 1;
            }
        }
        // Ahora el día final está en este mes
        self.dia += dias as i8;
    }
    pub fn restar_dias(&mut self, mut dias: i32) {
        if dias < 0 {
            self.sumar_dias(-dias);
            return
        }
        while dias >= self.dia as i32 {
            dias -= self.dia as i32;
            // Siempre que sea >=, hay que retroceder un mes. Nos paramos en el último día
            self.mes -= 1;
            if self.mes == 0 {
                self.mes = 12;
                self.año -= 1;
            }
            self.dia = self.ultimo_dia_mes();
        }
        // Ahora el día final está en este mes
        self.dia -= dias as i8;
    }

    pub fn es_mayor(&self, otra: &Self) -> bool {
        if self.año > otra.año {return true}
        if self.año < otra.año {return false}
        if self.mes > otra.mes {return true}
        if self.mes < otra.mes {return false}
        if self.dia > otra.dia {return true}
        false
    }
    pub fn igual_que(&self, otra: &Self) -> bool {
        self.año == otra.año && self.mes == otra.mes && self.dia == otra.dia
    }
}
// implementación de PartialEq (para que structs que lo usan lo puedan implementar) pero respetando el igual_que
impl PartialEq for Fecha {
    fn eq(&self, other: &Self) -> bool {
        self.igual_que(other)
    }
}


#[test]
fn fecha_test() {
    assert!(Fecha::new(10,11,2001).unwrap().igual_que(&Fecha {dia: 10, mes: 11, año: 2001}));
    assert!(Fecha::new(25,5,2023).unwrap().igual_que(&Fecha {dia: 25, mes: 5, año: 2023}));
    assert!(Fecha::new(10,13,2021).is_err());
    assert!(Fecha::new(10,12,2041).is_ok());
    assert!(Fecha::new(1,12,-2051).is_ok());
    assert!(Fecha::new(0,12,121).is_err());
    assert!(Fecha::new(-50,12,-50).is_err());
    assert!(Fecha::new(1,13,1999).is_err());
    let limites = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for (i, &limite) in limites.iter().enumerate() {
        assert!(Fecha::new(limite, i as i8 + 1, 1999).is_ok(), "dia: {limite} y mes: {i} debería ser válido");
        assert!(Fecha::new(limite+1, i as i8 + 1, 1999).is_err(), "dia: {limite} y mes: {i} debería ser inválido");
    }
    let bisiestos = [2000, 1600, 0, 2004, 2008, 2012, 2016, 2020, 2024, 1224];
    let no_bisiestos = [1700, 2100, 1900, 2001, 2002, 2003, 2023, 1223, 1234];
    for &bisiesto in bisiestos.iter() {
        assert!(Fecha::new(29, 2, bisiesto).is_ok(), "{bisiesto} es bisiesto");
        assert!(Fecha::new(1, 1, bisiesto).unwrap().es_bisiesto());
    }
    for &no_bisiesto in no_bisiestos.iter() {
        assert!(Fecha::new(29, 2, no_bisiesto).is_err(), "{no_bisiesto} no es bisiesto");
        assert!(!Fecha::new(1, 1, no_bisiesto).unwrap().es_bisiesto());
    }
    let fechas_ordenadas = [(1, 1, 2001), (2, 1, 2001), (5, 1, 2001), (1, 2, 2001), (1, 2, 2002), (3, 2, 2002), (1, 1, 2001)];
    for i in 0..fechas_ordenadas.len()-2 {
        let fecha_menor = fechas_ordenadas[i];
        let fecha_menor = Fecha::new(fecha_menor.0, fecha_menor.1, fecha_menor.2).unwrap();
        let fecha_mayor = fechas_ordenadas[i+1];
        let fecha_mayor = Fecha::new(fecha_mayor.0, fecha_mayor.1, fecha_mayor.2).unwrap();
        assert!(fecha_mayor.es_mayor(&fecha_menor));
        assert!(!fecha_menor.es_mayor(&fecha_mayor));
        assert!(fecha_mayor.igual_que(&fecha_mayor.clone()));
        assert!(fecha_menor.igual_que(&fecha_menor.clone()));
        assert!(!fecha_mayor.igual_que(&fecha_menor));
        assert!(!fecha_menor.igual_que(&fecha_mayor));
    }
    let mut fecha_inicial = Fecha::new(1, 1, 1999).unwrap();
    let sumas = vec![
        (1, (2, 1, 1999)),
        (30, (1, 2, 1999)),
        (3, (4, 2, 1999)),
        (33, (9, 3, 1999)),
        (33, (11, 4, 1999)),
        (33, (14, 5, 1999)),
        (33, (16, 6, 1999)),
        (0, (16, 6, 1999)),
        (413, (2, 8, 2000)),
        (413, (19, 9, 2001)),
        (413, (6, 11, 2002)),
        (1300, (29, 5, 2006)),
        (1300, (19, 12, 2009)),
        (1300, (11, 7, 2013)),
        (20, (31, 7, 2013)),
        (-20, (11, 7, 2013)),
        (21, (1, 8, 2013)),
        (-31, (1, 7, 2013)),
        (31, (1, 8, 2013)),
        (3584, (25, 5, 2023)),
    ];
    for suma in sumas.iter() {
        let orig = fecha_inicial.clone();
        let resultado = Fecha::new(suma.1.0, suma.1.1, suma.1.2).unwrap();
        fecha_inicial.sumar_dias(suma.0);
        assert!(fecha_inicial.igual_que(&resultado), "{orig:?} + {} dias, se esperaba {resultado:?} pero dio {fecha_inicial:?}", suma.0);
    }
    for resta in sumas.iter().rev() {
        let resultado = Fecha::new(resta.1.0, resta.1.1, resta.1.2).unwrap();
        assert!(fecha_inicial.igual_que(&resultado), "{fecha_inicial:?} != {resultado:?}");
        fecha_inicial.restar_dias(resta.0);
    }
    assert!(fecha_inicial.igual_que(&Fecha::new(1, 1, 1999).unwrap()));
}