// Zadanie 1
#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum SurowiecType {
    Drewno,
    Glina,
    Zelazo,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Surowiec {
    surowiec: SurowiecType,
    ilosc: u32,
    nazwa_ilosciowa: String,
}

impl Surowiec {
    pub fn new(surowiec: SurowiecType, ilosc: u32) -> Surowiec {
        match surowiec {
            SurowiecType::Zelazo => {
                Surowiec {surowiec, ilosc, nazwa_ilosciowa:"sztabek".to_string()}
            }
            SurowiecType::Glina => {
                Surowiec {surowiec, ilosc, nazwa_ilosciowa:"ton".to_string()}
            }
            SurowiecType::Drewno => {
                Surowiec {surowiec, ilosc, nazwa_ilosciowa:"ton".to_string()}
            }
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum TransportType {
    Ciezarowka,
    Pociag,
}

// Zadanie 2
#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Magazyn {
    adres: String,
    pojemnosc_w_tonach: u32,
    czy_posiada_terminal: bool,
}

impl Magazyn {
    pub fn new(adres: String, pojemnosc_w_tonach: u32, czy_posiada_terminal: bool) -> Magazyn {
        Magazyn {adres, pojemnosc_w_tonach, czy_posiada_terminal}
    }

    pub fn get_adres(self) -> String {
        let adres = self.adres.clone();
        adres
    }

    pub fn set_adres(&mut self, adres: String) {
        self.adres = adres;
    }

    pub fn get_pojemnosc_w_tonach(self) -> u32 {
        let pojemnosc_w_tonach = self.pojemnosc_w_tonach.clone();
        pojemnosc_w_tonach
    }

    pub fn set_pojemnosc_w_tonach(&mut self, pojemnosc_w_tonach: u32) {
        self.pojemnosc_w_tonach = pojemnosc_w_tonach;
    }

    pub fn get_czy_posiada_terminal(self) -> bool {
        let czy_posiada_terminal = self.czy_posiada_terminal.clone();
        czy_posiada_terminal
    }

    pub fn set_czy_posiada_termina(&mut self, czy_posiada_terminal: bool) {
        self.czy_posiada_terminal = czy_posiada_terminal;
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Przewoz {
    adres_poczatkowy: String,
    adres_docelowy: String,
    surowiec: Surowiec,
    srodek_transportu: TransportType
}

impl Przewoz {
    pub fn new(adres_poczatkowy: String, adres_docelowy: String, surowiec: Surowiec, srodek_transportu: TransportType) -> Przewoz {
        Przewoz {adres_poczatkowy, adres_docelowy, surowiec, srodek_transportu}
    }

    pub fn get_adres_poczatkowy(self) -> String {
        let adres_poczatkowy = self.adres_poczatkowy.clone();
        adres_poczatkowy
    }

    pub fn set_adres_poczatkowy(&mut self, adres_poczatkowy: String) {
        self.adres_poczatkowy = adres_poczatkowy;
    }

    pub fn get_adres_docelowy(self) -> String {
        let adres_docelowy = self.adres_docelowy.clone();
        adres_docelowy
    }

    pub fn set_adres_docelowy(&mut self, adres_docelowy: String) {
        self.adres_docelowy = adres_docelowy;
    }

    pub fn get_surowiec(self) -> Surowiec {
        let surowiec = self.surowiec.clone();
        surowiec
    }

    pub fn set_surowiec(&mut self, surowiec: Surowiec) {
        self.surowiec = surowiec
    }

    pub fn get_srodek_transportu(self) -> TransportType {
        let srodek_transportu = self.srodek_transportu.clone();
        srodek_transportu
    }

    pub fn set_srodek_transportu(&mut self, srodek_transportu: TransportType) {
        self.srodek_transportu = srodek_transportu;
    }

    pub fn zaladuj(&mut self, surowiec: Surowiec, srodek_transportu: TransportType) {
        self.surowiec = surowiec;
        self.srodek_transportu = srodek_transportu;
    }

    pub fn dostawa_do(self, magazyn: Magazyn) -> bool {
        let mut ilosc_surowca = self.surowiec.ilosc.clone();
        if self.surowiec.surowiec == SurowiecType::Zelazo {
            ilosc_surowca *= 5;
        }

        if ilosc_surowca < magazyn.clone().get_pojemnosc_w_tonach() {
            if self.srodek_transportu == TransportType::Pociag {
               if magazyn.clone().get_czy_posiada_terminal() == true {
                    println!("{}", self.adres_docelowy);
                    return true;
                }
            }
        }
        return false;
    }
}

fn main() {
    // Zadanie 1
    println!("Zadanie 1");
    let mut surowiec = Surowiec::new(SurowiecType::Zelazo, 1000);
    println!("{:?}", surowiec);

    // Zadanie 2
    println!("Zadanie 2");
    let mut magazyn = Magazyn::new("ul. Lipowa 7, Lublin".to_string(), 1000, true);
    println!("{:?}", magazyn);
    magazyn.set_adres("ul. Akademicka 7, Lublin".to_string());
    magazyn.set_pojemnosc_w_tonach(2000);
    // magazyn.set_czy_posiada_terminal(false);
    println!("{}", magazyn.clone().get_adres());
    println!("{}", magazyn.clone().get_pojemnosc_w_tonach());
    println!("{}", magazyn.clone().get_czy_posiada_terminal());

    // Zadanie 3
    println!("Zadanie 3");
    let mut przewoz = Przewoz::new("ul. Lipowa 7, Lublin".to_string(), "ul. Lipowa 7, Krakow".to_string(), surowiec.clone(), TransportType::Pociag);
    przewoz.zaladuj(surowiec.clone(), TransportType::Pociag);
    println!("{:?}", przewoz.dostawa_do(magazyn));
}
