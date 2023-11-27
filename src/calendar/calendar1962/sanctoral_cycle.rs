use super::*;

// TODO: copied from 1939

impl Calendar1962 {
    pub fn moveable_feasts<'a>(self, year: i32) -> Vec<CalendarEntry<'a>> {
        let mut entries: Vec<CalendarEntry<'a>> = Vec::new();
        let ch = CalendarHelper::new(year);
        let (holy_family_m, holy_family_d) = ch.month_day(ch.sunday_after(ch.epiphany()).unwrap());

        let holy_family_d = if holy_family_d == 13 {
            12
        } else {
            holy_family_d
        };
        entries.push((
            holy_family_m,
            holy_family_d,
            FeastDetails::new("s-familiae-jmj", FeastRank::GreaterDouble)
                .with_person(Person::OurLord),
        ));

        let purification = ch.ordinal0(2, 2);
        let purification_d = if ch.is_sunday(purification) && purification >= ch.septuagesima() {
            3
        } else {
            2
        };
        // TODO: feast of Our Lord and Our Lady
        entries.push((
            2,
            purification_d,
            FeastDetails::new("in-purificatione-bmv", FeastRank::DoubleSecondClass)
                .with_proper_date(2, 2)
                .with_person(Person::OurLord)
                .make_feriatum(),
        ));

        let annunciation = ch.ordinal0(3, 25);
        let annunciation = if annunciation >= ch.easter() - 7 {
            ch.easter() + 8
        } else if ch.is_sunday(annunciation) {
            annunciation + 1
        } else {
            annunciation
        };
        let (annunciation_m, annunciation_d) = ch.month_day(annunciation);
        entries.push((
            annunciation_m,
            annunciation_d,
            FeastDetails::new("in-annuntiatione-bmv", FeastRank::DoubleFirstClass)
                .with_proper_date(3, 25)
                .with_person(Person::OurLady)
                .make_feriatum(),
        ));

        let (sorrows_m, sorrows_d) = ch.month_day(ch.easter() - 9);
        entries.push((
            sorrows_m,
            sorrows_d,
            FeastDetails::new("septem-dolorum-bmv-temp-pass", FeastRank::GreaterDouble)
                .with_person(Person::OurLady)
                .make_secondary(),
        ));

        let (joseph_m, joseph_d) = ch.month_day(ch.easter() + 17);
        entries.push((
            joseph_m,
            joseph_d,
            FeastDetails::new("solemnitas-s-joseph", FeastRank::DoubleFirstClass)
                .with_person(Person::StJoseph)
                .with_octave(OctaveRank::Common)
                .make_feriatum(),
        ));

        let ctk = ch.sunday_after(ch.ordinal0(10, 24)).unwrap();
        let (ctk_m, ctk_d) = ch.month_day(ctk);
        entries.push((
            ctk_m,
            ctk_d,
            FeastDetails::new("dnjc-regis", FeastRank::DoubleFirstClass)
                .with_person(Person::OurLord),
        ));

        entries
    }
}

pub static CALENDAR_OF_SAINTS: [CalendarEntry; 318] = [
    (
        1,
        1,
        FeastDetails::new("in-circumcisione-dnjc", FeastRank::DoubleSecondClass)
            .with_person(Person::OurLord)
            .make_feriatum(),
    ),
    (
        1,
        5,
        FeastDetails::new("s-telesphori-pm", FeastRank::Commemoration),
    ),
    (
        1,
        6,
        FeastDetails::new("in-epiphania-dnjc", FeastRank::DoubleFirstClass)
            .with_person(Person::OurLord)
            .make_feriatum()
            .with_vigil(VigilRank::SecondClass)
            .with_octave(OctaveRank::SecondOrder),
    ),
    (
        1,
        11,
        FeastDetails::new("s-hygini-pm", FeastRank::Commemoration),
    ),
    (1, 14, FeastDetails::new("s-hilarii-ecd", FeastRank::Double)),
    (
        1,
        14,
        FeastDetails::new("s-felicis-m", FeastRank::Commemoration),
    ),
    (
        1,
        15,
        FeastDetails::new("s-pauli-primi-erimitae-c", FeastRank::Double),
    ),
    (
        1,
        15,
        FeastDetails::new("s-mauri-abb", FeastRank::Commemoration),
    ),
    (
        1,
        16,
        FeastDetails::new("s-marcelli-pm", FeastRank::Semidouble),
    ),
    (1, 17, FeastDetails::new("s-antonii-abb", FeastRank::Double)),
    // TODO: commemoration of St Paul
    (
        1,
        18,
        FeastDetails::new("cathedra-s-petri-romae", FeastRank::GreaterDouble).make_secondary(),
    ),
    (
        1,
        18,
        FeastDetails::new("s-priscae-vm", FeastRank::Commemoration),
    ),
    (
        1,
        19,
        FeastDetails::new("ss-marii-et-soc-mm", FeastRank::Simple),
    ),
    (
        1,
        19,
        FeastDetails::new("s-canuti-regis", FeastRank::Commemoration),
    ),
    (
        1,
        20,
        FeastDetails::new("ss-fabiani-p-et-sabastiani-mm", FeastRank::Double),
    ),
    (1, 21, FeastDetails::new("s-agnetis-vm", FeastRank::Double)),
    (
        1,
        22,
        FeastDetails::new("ss-vincentii-et-anastasii-mm", FeastRank::Semidouble),
    ),
    (
        1,
        23,
        FeastDetails::new("s-raymundi-de-penafort-c", FeastRank::Semidouble),
    ),
    (
        1,
        23,
        FeastDetails::new("s-emerentianae-vm", FeastRank::Commemoration),
    ),
    (1, 24, FeastDetails::new("s-timothei-em", FeastRank::Double)),
    // TODO: commemoration of St Peter
    (
        1,
        25,
        FeastDetails::new("conversio-s-pauli-ap", FeastRank::GreaterDouble).make_secondary(),
    ),
    (
        1,
        26,
        FeastDetails::new("s-polycarpi-em", FeastRank::Double),
    ),
    (
        1,
        27,
        FeastDetails::new("s-joannis-chrysostomi-ecd", FeastRank::Double),
    ),
    (
        1,
        28,
        FeastDetails::new("s-petri-nolasci-c", FeastRank::Double),
    ),
    (
        1,
        28,
        FeastDetails::new("s-agnetis-vm-secundo", FeastRank::Commemoration),
    ),
    (
        1,
        29,
        FeastDetails::new("s-francisci-salesii-ecd", FeastRank::Double),
    ),
    (
        1,
        30,
        FeastDetails::new("s-martinae-vm", FeastRank::Semidouble),
    ),
    (
        1,
        31,
        FeastDetails::new("s-joannis-bosco-c", FeastRank::Double),
    ),
    (2, 1, FeastDetails::new("s-ignatii-em", FeastRank::Double)),
    // 2/2 Purification: handled as a special case because of its transfer rule
    (2, 3, FeastDetails::new("s-blasii-em", FeastRank::Simple)),
    (
        2,
        4,
        FeastDetails::new("s-andreae-corsini-ec", FeastRank::Double),
    ),
    (2, 5, FeastDetails::new("s-agathae-vm", FeastRank::Double)),
    (2, 6, FeastDetails::new("s-titi-ec", FeastRank::Double)),
    (
        2,
        6,
        FeastDetails::new("s-dorotheae-vm", FeastRank::Commemoration),
    ),
    (2, 7, FeastDetails::new("s-romualdi-abb", FeastRank::Double)),
    (
        2,
        8,
        FeastDetails::new("s-joannis-de-matha-c", FeastRank::Double),
    ),
    (
        2,
        9,
        FeastDetails::new("s-cyrilli-e-alexandrini-cd", FeastRank::Double),
    ),
    (
        2,
        9,
        FeastDetails::new("s-apolloniae-vm", FeastRank::Commemoration),
    ),
    (
        2,
        10,
        FeastDetails::new("s-scholasticae-v", FeastRank::Double),
    ),
    (
        2,
        11,
        FeastDetails::new("apparitio-bmv-immaculatae", FeastRank::GreaterDouble)
            .with_person(Person::OurLady)
            .make_secondary(),
    ),
    (
        2,
        12,
        FeastDetails::new(
            "ss-septem-fundatorum-ordinis-servorum-bmv-cc",
            FeastRank::Double,
        ),
    ),
    (2, 14, FeastDetails::new("s-valentini-m", FeastRank::Simple)),
    (
        2,
        15,
        FeastDetails::new("ss-faustini-et-jovitae-mm", FeastRank::Simple),
    ),
    (2, 18, FeastDetails::new("s-simeonis-em", FeastRank::Simple)),
    // TODO: comm of St Paul
    (
        2,
        22,
        FeastDetails::new("cathedra-s-petri-antiochiae", FeastRank::GreaterDouble).make_secondary(),
    ),
    (
        2,
        23,
        FeastDetails::new("s-petri-damiani-ecd", FeastRank::Double),
    ),
    (
        2,
        24,
        FeastDetails::new("s-matthiae-ap", FeastRank::DoubleSecondClass)
            .with_person(Person::Apostle)
            .with_vigil(VigilRank::Common)
            .make_feriatum(),
    ),
    (
        2,
        27,
        FeastDetails::new("s-gabrielis-a-virgine-perdolente-c", FeastRank::Double),
    ),
    (
        3,
        4,
        FeastDetails::new("s-casimiri-c", FeastRank::Semidouble),
    ),
    (
        3,
        4,
        FeastDetails::new("s-lucii-i-pm", FeastRank::Commemoration),
    ),
    (
        3,
        6,
        FeastDetails::new("ss-perpetua-et-felicitatis-mm", FeastRank::Double),
    ),
    (
        3,
        7,
        FeastDetails::new("s-thomae-de-aquina-cd", FeastRank::Double),
    ),
    (
        3,
        8,
        FeastDetails::new("s-joannis-de-deo-c", FeastRank::Double),
    ),
    (
        3,
        9,
        FeastDetails::new("s-franciscae-romanae-vid", FeastRank::Double),
    ),
    (
        3,
        10,
        FeastDetails::new("ss-quadraginta-mm", FeastRank::Semidouble),
    ),
    (
        3,
        12,
        FeastDetails::new("s-gregorii-i-pcd", FeastRank::Double),
    ),
    (3, 17, FeastDetails::new("s-patricii-ec", FeastRank::Double)),
    (
        3,
        18,
        FeastDetails::new("s-cyrilli-e-hierosolymitani-cd", FeastRank::Double),
    ),
    (
        3,
        19,
        FeastDetails::new("s-joseph-sponsi-bmv-c", FeastRank::DoubleFirstClass)
            .with_person(Person::StJoseph)
            .make_feriatum(),
    ),
    (
        3,
        21,
        FeastDetails::new("s-benedicti-abb", FeastRank::GreaterDouble),
    ),
    (
        3,
        24,
        FeastDetails::new("s-gabrielis-archangeli", FeastRank::GreaterDouble),
    ),
    // 3/25 the Annunciation is covered as a special case
    (
        3,
        27,
        FeastDetails::new("s-joannis-damasceni-cd", FeastRank::Double),
    ),
    (
        3,
        28,
        FeastDetails::new("s-joannis-a-capistrano-c", FeastRank::Semidouble),
    ),
    (
        4,
        2,
        FeastDetails::new("s-francisci-de-paula-c", FeastRank::Double),
    ),
    (4, 4, FeastDetails::new("s-isidori-ecd", FeastRank::Double)),
    (
        4,
        5,
        FeastDetails::new("s-vincentii-ferrerii-c", FeastRank::Double),
    ),
    (
        4,
        11,
        FeastDetails::new("s-leonis-i-pcd", FeastRank::Double),
    ),
    (
        4,
        13,
        FeastDetails::new("s-hermenegildi-regis-m", FeastRank::Semidouble),
    ),
    (4, 14, FeastDetails::new("s-justini-m", FeastRank::Double)),
    (
        4,
        14,
        FeastDetails::new("ss-tiburtii-et-soc-mm", FeastRank::Commemoration),
    ),
    (4, 17, FeastDetails::new("s-aniceti-pm", FeastRank::Simple)),
    (4, 21, FeastDetails::new("s-anselmi-ecd", FeastRank::Double)),
    (
        4,
        22,
        FeastDetails::new("ss-soteris-et-caji-pp", FeastRank::Semidouble),
    ),
    (
        4,
        23,
        FeastDetails::new("s-georgii-m", FeastRank::Semidouble),
    ),
    (
        4,
        24,
        FeastDetails::new("s-fidelis-a-sigmaringa-m", FeastRank::Double),
    ),
    (
        4,
        25,
        FeastDetails::new("s-marci-ev", FeastRank::DoubleSecondClass).with_person(Person::Apostle),
    ),
    (
        4,
        26,
        FeastDetails::new("ss-cleti-et-marcellini-pp-mm", FeastRank::Semidouble),
    ),
    (
        4,
        27,
        FeastDetails::new("s-petri-canisii-cd", FeastRank::Double),
    ),
    (
        4,
        28,
        FeastDetails::new("s-pauli-a-cruce-c", FeastRank::Double),
    ),
    (
        4,
        28,
        FeastDetails::new("s-vitalis-m", FeastRank::Commemoration),
    ),
    (4, 29, FeastDetails::new("s-petri-m", FeastRank::Double)),
    (
        4,
        30,
        FeastDetails::new("s-catharinae-senensis-v", FeastRank::Double),
    ),
    (
        5,
        1,
        FeastDetails::new("ss-philippi-et-jacobi-app", FeastRank::DoubleSecondClass)
            .with_person(Person::Apostle),
    ),
    (
        5,
        2,
        FeastDetails::new("s-athanasii-ecd", FeastRank::Double),
    ),
    (
        5,
        3,
        FeastDetails::new("inventio-s-crucis", FeastRank::DoubleSecondClass)
            .make_secondary()
            .make_feriatum(),
    ),
    // TODO: according to SLP this isn't commemorated at 1V; why?
    (
        5,
        3,
        FeastDetails::new("ss-alexandri-i-p-et-soc-mm-ec", FeastRank::Commemoration),
    ),
    (5, 4, FeastDetails::new("s-monicae-vid", FeastRank::Double)),
    (5, 5, FeastDetails::new("s-pii-v-pc", FeastRank::Double)),
    (
        5,
        6,
        FeastDetails::new("s-joannis-ante-portam-latinam", FeastRank::GreaterDouble)
            .with_person(Person::Apostle)
            .make_secondary(),
    ),
    (5, 7, FeastDetails::new("s-stanislai-em", FeastRank::Double)),
    (
        5,
        8,
        FeastDetails::new("apparitio-s-michaelis-archangeli", FeastRank::GreaterDouble)
            .with_person(Person::Angel)
            .make_secondary(),
    ),
    (
        5,
        9,
        FeastDetails::new("s-gregorii-nazianzeni-ecd", FeastRank::Double),
    ),
    (5, 10, FeastDetails::new("s-antonini-ec", FeastRank::Double)),
    (
        5,
        10,
        FeastDetails::new("ss-gordiani-et-epimachi-mm", FeastRank::Commemoration),
    ),
    (
        5,
        12,
        FeastDetails::new("ss-nerei-et-soc-mm", FeastRank::Semidouble),
    ),
    (
        5,
        13,
        FeastDetails::new("s-roberti-bellarmino-ecd", FeastRank::Double),
    ),
    (5, 14, FeastDetails::new("s-bonifatii-m", FeastRank::Simple)),
    (
        5,
        15,
        FeastDetails::new("s-joannis-baptistae-de-la-salle-c", FeastRank::Double),
    ),
    (
        5,
        16,
        FeastDetails::new("s-ubaldi-ec", FeastRank::Semidouble),
    ),
    (
        5,
        17,
        FeastDetails::new("s-paschalis-baylon-c", FeastRank::Double),
    ),
    (5, 18, FeastDetails::new("s-venantii-m", FeastRank::Double)),
    (
        5,
        19,
        FeastDetails::new("s-petri-caelestini-pc", FeastRank::Double),
    ),
    (
        5,
        19,
        FeastDetails::new("s-pudentianae-v", FeastRank::Commemoration),
    ),
    (
        5,
        20,
        FeastDetails::new("s-bernardini-senensis-c", FeastRank::Semidouble),
    ),
    (
        5,
        25,
        FeastDetails::new("s-gregorii-vii-pc", FeastRank::Double),
    ),
    (
        5,
        25,
        FeastDetails::new("s-urbani-i-pm", FeastRank::Commemoration),
    ),
    (
        5,
        26,
        FeastDetails::new("s-philippi-nerii-c", FeastRank::Double),
    ),
    (
        5,
        26,
        FeastDetails::new("s-eleutherii-pm", FeastRank::Commemoration),
    ),
    (
        5,
        27,
        FeastDetails::new("s-bedae-venerabilis-cd", FeastRank::Double),
    ),
    (
        5,
        27,
        FeastDetails::new("s-joannis-i-pm", FeastRank::Commemoration),
    ),
    (
        5,
        28,
        FeastDetails::new("s-augustini-ec", FeastRank::Double),
    ),
    (
        5,
        29,
        FeastDetails::new("s-mariae-magdalenae-de-pazzis-v", FeastRank::Semidouble),
    ),
    (5, 30, FeastDetails::new("s-felicis-pm", FeastRank::Simple)),
    (
        5,
        31,
        FeastDetails::new("s-angelae-mericiae-v", FeastRank::Double),
    ),
    (
        5,
        31,
        FeastDetails::new("s-petronillae-v", FeastRank::Commemoration),
    ),
    (
        6,
        2,
        FeastDetails::new("ss-marcellini-et-soc-mm", FeastRank::Simple),
    ),
    (
        6,
        4,
        FeastDetails::new("s-francisci-caracciolo-c", FeastRank::Double),
    ),
    (6, 5, FeastDetails::new("s-bonifatii-em", FeastRank::Double)),
    (6, 6, FeastDetails::new("s-norbertii-ec", FeastRank::Double)),
    (
        6,
        9,
        FeastDetails::new("ss-primi-et-feliciani-mm", FeastRank::Simple),
    ),
    (
        6,
        10,
        FeastDetails::new("s-margaritae-reginae-vid", FeastRank::Semidouble),
    ),
    (
        6,
        11,
        FeastDetails::new("s-barnabae-ap", FeastRank::GreaterDouble).with_person(Person::Apostle),
    ),
    (
        6,
        12,
        FeastDetails::new("s-joannis-a-s-facundo-c", FeastRank::Double),
    ),
    (
        6,
        12,
        FeastDetails::new("ss-basilidis-et-soc-mm", FeastRank::Commemoration),
    ),
    (
        6,
        13,
        FeastDetails::new("s-antonii-de-padua-c", FeastRank::Double),
    ),
    (6, 14, FeastDetails::new("s-basilii-ecd", FeastRank::Double)),
    (
        6,
        15,
        FeastDetails::new("ss-viti-et-soc-mm", FeastRank::Simple),
    ),
    (
        6,
        18,
        FeastDetails::new("s-ephraem-syri-cd", FeastRank::Double),
    ),
    (
        6,
        18,
        FeastDetails::new("ss-marci-et-marcelliani-mm", FeastRank::Commemoration),
    ),
    (
        6,
        19,
        FeastDetails::new("s-julianae-de-falconeriis-v", FeastRank::Double),
    ),
    (
        6,
        19,
        FeastDetails::new("ss-gervasii-et-protasii-mm", FeastRank::Commemoration),
    ),
    (6, 20, FeastDetails::new("s-silverii-pm", FeastRank::Simple)),
    (
        6,
        21,
        FeastDetails::new("s-aloysii-gonzagae-c", FeastRank::Double),
    ),
    (6, 22, FeastDetails::new("s-paulini-ec", FeastRank::Double)),
    (
        6,
        24,
        FeastDetails::new("nativitas-s-joannis-baptistae", FeastRank::DoubleFirstClass)
            .with_person(Person::StJohnBaptist)
            .with_vigil(VigilRank::Common)
            .with_octave(OctaveRank::Common)
            .make_feriatum(),
    ),
    (
        6,
        25,
        FeastDetails::new("s-gulielmi-abb", FeastRank::Double),
    ),
    (
        6,
        26,
        FeastDetails::new("ss-joannnis-et-pauli-mm", FeastRank::Double),
    ),
    (6, 28, FeastDetails::new("s-irenaei-em", FeastRank::Double)),
    (
        6,
        29,
        FeastDetails::new("ss-petri-et-pauli-app", FeastRank::DoubleFirstClass)
            .with_person(Person::Apostle)
            .with_vigil(VigilRank::Common)
            .with_octave(OctaveRank::Common)
            .make_feriatum(),
    ),
    // TODO: commemoration of St Peter
    (
        6,
        30,
        FeastDetails::new("commemoratio-s-pauli-ap", FeastRank::GreaterDouble)
            .with_person(Person::Apostle)
            .make_secondary(),
    ),
    (
        7,
        1,
        FeastDetails::new("pretiosissimi-sanguinis-dnjc", FeastRank::DoubleFirstClass)
            .with_person(Person::OurLord)
            .make_secondary(),
    ),
    (
        7,
        2,
        FeastDetails::new("visitatio-bmv", FeastRank::DoubleSecondClass)
            .with_person(Person::OurLady),
    ),
    (
        7,
        3,
        FeastDetails::new("s-leonis-ii-pc", FeastRank::Semidouble),
    ),
    (
        7,
        5,
        FeastDetails::new("s-antonii-mariae-zaccaria-c", FeastRank::Double),
    ),
    (
        7,
        7,
        FeastDetails::new("ss-cyrilli-et-methodii-epp-cc", FeastRank::Double),
    ),
    (
        7,
        8,
        FeastDetails::new("s-elisabeth-reginae-vid", FeastRank::Semidouble),
    ),
    (
        7,
        10,
        FeastDetails::new(
            "ss-septem-fratrum-mm-et-ss-rufinae-et-secundae-vv-mm",
            FeastRank::Semidouble,
        ),
    ),
    (7, 11, FeastDetails::new("s-pii-i-pm", FeastRank::Simple)),
    (
        7,
        12,
        FeastDetails::new("s-joannis-gualberti-abb", FeastRank::Double),
    ),
    (
        7,
        12,
        FeastDetails::new("ss-naboris-et-felicis-mm", FeastRank::Commemoration),
    ),
    (
        7,
        13,
        FeastDetails::new("s-anacleti-pm", FeastRank::Semidouble),
    ),
    (
        7,
        14,
        FeastDetails::new("s-bonaventurae-ecd", FeastRank::Double),
    ),
    (
        7,
        15,
        FeastDetails::new("s-henrici-imperatoris-c", FeastRank::Semidouble),
    ),
    (
        7,
        16,
        FeastDetails::new(
            "commemoratio-bmv-de-monte-carmelo",
            FeastRank::GreaterDouble,
        )
        .with_person(Person::OurLady)
        .make_secondary(),
    ),
    (
        7,
        17,
        FeastDetails::new("s-alexii-c", FeastRank::Semidouble),
    ),
    (
        7,
        18,
        FeastDetails::new("s-camilli-de-lellis-c", FeastRank::Double),
    ),
    (
        7,
        18,
        FeastDetails::new(
            "ss-symphorosae-et-septem-filiorum-ejus-mm",
            FeastRank::Commemoration,
        ),
    ),
    (
        7,
        19,
        FeastDetails::new("s-vincentii-a-paulo-c", FeastRank::Double),
    ),
    (
        7,
        20,
        FeastDetails::new("s-hieronymi-aemiliani-c", FeastRank::Double),
    ),
    (
        7,
        20,
        FeastDetails::new("s-margaritae-vm", FeastRank::Commemoration),
    ),
    (7, 21, FeastDetails::new("s-praxedis-v", FeastRank::Simple)),
    (
        7,
        22,
        FeastDetails::new("s-mariae-magdalenae-poenitentis", FeastRank::Double),
    ),
    (
        7,
        23,
        FeastDetails::new("s-apollinaris-em", FeastRank::Double),
    ),
    (
        7,
        23,
        FeastDetails::new("s-liborii-ec", FeastRank::Commemoration),
    ),
    (
        7,
        24,
        FeastDetails::new("s-christinae-vm", FeastRank::Commemoration),
    ),
    (
        7,
        25,
        FeastDetails::new("s-jacobi-ap", FeastRank::DoubleSecondClass)
            .with_person(Person::Apostle)
            .with_vigil(VigilRank::Common)
            .make_feriatum(),
    ),
    (
        7,
        26,
        FeastDetails::new("s-annae-matris-bmv", FeastRank::DoubleSecondClass).make_feriatum(),
    ),
    (
        7,
        27,
        FeastDetails::new("s-pantaleonis-m", FeastRank::Simple),
    ),
    (
        7,
        28,
        FeastDetails::new(
            "ss-nazarii-et-celsi-mm-et-victoris-i-pm-ac-innocentii-pc",
            FeastRank::Semidouble,
        ),
    ),
    (
        7,
        29,
        FeastDetails::new("s-marthae-v", FeastRank::Semidouble),
    ),
    (
        7,
        29,
        FeastDetails::new("ss-felicis-ii-p-et-soc-mm", FeastRank::Commemoration),
    ),
    (
        7,
        30,
        FeastDetails::new("ss-abdon-et-sennen-mm", FeastRank::Simple),
    ),
    (
        7,
        31,
        FeastDetails::new("s-ignatii-c", FeastRank::GreaterDouble),
    ),
    (
        8,
        1,
        FeastDetails::new("s-petri-ad-vincula", FeastRank::GreaterDouble)
            .with_person(Person::Apostle),
    ),
    (
        8,
        1,
        FeastDetails::new("ss-machabaeorum-mm", FeastRank::Commemoration),
    ),
    (
        8,
        2,
        FeastDetails::new("s-alfonsi-mariae-de-ligorio-ecd", FeastRank::Double),
    ),
    (
        8,
        2,
        FeastDetails::new("s-stephani-i-pm", FeastRank::Commemoration),
    ),
    (
        8,
        3,
        FeastDetails::new("inventio-s-stephani-protomartyris", FeastRank::Semidouble)
            .make_secondary(),
    ),
    (
        8,
        4,
        FeastDetails::new("s-dominici-c", FeastRank::GreaterDouble),
    ),
    (
        8,
        5,
        FeastDetails::new("dedicatio-s-mariae-ad-nives", FeastRank::GreaterDouble)
            .with_person(Person::OurLady),
    ),
    (
        8,
        6,
        FeastDetails::new("transfiguratio-dnjc", FeastRank::DoubleSecondClass)
            .with_person(Person::OurLord),
    ),
    (
        8,
        6,
        FeastDetails::new(
            "ss-xysti-ii-p-felicissimi-et-agapiti-mm",
            FeastRank::Commemoration,
        ),
    ),
    (8, 7, FeastDetails::new("s-cajetani-c", FeastRank::Double)),
    (
        8,
        7,
        FeastDetails::new("s-donati-em", FeastRank::Commemoration),
    ),
    (
        8,
        8,
        FeastDetails::new("ss-cyriaci-largi-et-smaragdi-mm", FeastRank::Semidouble),
    ),
    (
        8,
        9,
        FeastDetails::new("s-joannis-mariae-vianney", FeastRank::Double),
    ),
    (
        8,
        9,
        FeastDetails::new("s-romani-m", FeastRank::Commemoration),
    ),
    (
        8,
        10,
        FeastDetails::new("s-laurentii-m", FeastRank::DoubleSecondClass)
            .with_vigil(VigilRank::Common)
            .with_octave(OctaveRank::Simple)
            .make_feriatum(),
    ),
    (
        8,
        11,
        FeastDetails::new("ss-tiburtii-et-susannae-v-mm", FeastRank::Simple),
    ),
    (8, 12, FeastDetails::new("s-clarae-v", FeastRank::Double)),
    (
        8,
        13,
        FeastDetails::new("ss-hippolyti-et-cassiani-mm", FeastRank::Simple),
    ),
    (
        8,
        14,
        FeastDetails::new("s-eusebii-c", FeastRank::Commemoration),
    ),
    (
        8,
        15,
        FeastDetails::new("assumptio-bmv", FeastRank::DoubleFirstClass)
            .with_person(Person::OurLady)
            .with_vigil(VigilRank::Common)
            .with_octave(OctaveRank::Common)
            .make_feriatum(),
    ),
    (
        8,
        16,
        FeastDetails::new("s-joachim-patris-bmv-c", FeastRank::DoubleSecondClass).make_feriatum(),
    ),
    (8, 17, FeastDetails::new("s-hyacinthi-c", FeastRank::Double)),
    (
        8,
        18,
        FeastDetails::new("s-agapiti-m", FeastRank::Commemoration),
    ),
    (
        8,
        19,
        FeastDetails::new("s-joannis-eudes-c", FeastRank::Double),
    ),
    (
        8,
        20,
        FeastDetails::new("s-bernardi-abb-cd", FeastRank::Double),
    ),
    (
        8,
        21,
        FeastDetails::new(
            "s-joannae-franciscae-fremiot-de-chantal-vid",
            FeastRank::Double,
        ),
    ),
    (
        8,
        22,
        FeastDetails::new("ss-timothei-et-soc-mm", FeastRank::Commemoration),
    ),
    (
        8,
        23,
        FeastDetails::new("s-philippi-benitii-c", FeastRank::Double),
    ),
    (
        8,
        24,
        FeastDetails::new("s-bartholomaei-ap", FeastRank::DoubleSecondClass)
            .with_person(Person::Apostle)
            .with_vigil(VigilRank::Common)
            .make_feriatum(),
    ),
    (
        8,
        25,
        FeastDetails::new("s-ludovici-regis-c", FeastRank::Semidouble),
    ),
    (
        8,
        26,
        FeastDetails::new("s-zephyrini-pm", FeastRank::Simple),
    ),
    (
        8,
        27,
        FeastDetails::new("s-josephi-calasanctii-c", FeastRank::Double),
    ),
    (
        8,
        28,
        FeastDetails::new("s-augustini-ecd", FeastRank::Double),
    ),
    (
        8,
        28,
        FeastDetails::new("s-hermetis-m", FeastRank::Commemoration),
    ),
    (
        8,
        29,
        FeastDetails::new("decollatio-s-joannis-baptistae", FeastRank::GreaterDouble)
            .with_person(Person::StJohnBaptist),
    ),
    (
        8,
        29,
        FeastDetails::new("s-sabinae-m", FeastRank::Commemoration),
    ),
    (
        8,
        30,
        FeastDetails::new("s-rosae-limanae-v", FeastRank::Double),
    ),
    (
        8,
        30,
        FeastDetails::new("ss-felicis-et-audacti-mm", FeastRank::Commemoration),
    ),
    (
        8,
        31,
        FeastDetails::new("s-raymundi-nonnati-c", FeastRank::Double),
    ),
    (9, 1, FeastDetails::new("s-aegidii-abb", FeastRank::Simple)),
    (
        9,
        1,
        FeastDetails::new("ss-duodecim-fratrum-mm", FeastRank::Commemoration),
    ),
    (
        9,
        2,
        FeastDetails::new("s-stephani-regis-c", FeastRank::Semidouble),
    ),
    (
        9,
        5,
        FeastDetails::new("s-laurentii-justiniani-ec", FeastRank::Semidouble),
    ),
    (
        9,
        8,
        FeastDetails::new("nativitas-bmv", FeastRank::DoubleSecondClass)
            .with_person(Person::OurLady)
            .with_octave(OctaveRank::Simple)
            .make_feriatum(),
    ),
    (9, 9, FeastDetails::new("s-gorgonii-m", FeastRank::Simple)),
    (
        9,
        10,
        FeastDetails::new("s-nicolai-de-tolentino-c", FeastRank::Double),
    ),
    (
        9,
        11,
        FeastDetails::new("ss-proti-et-hyacinthi-mm", FeastRank::Simple),
    ),
    (
        9,
        12,
        FeastDetails::new("ss-nominis-mariae", FeastRank::GreaterDouble)
            .with_person(Person::OurLady)
            .make_secondary(),
    ),
    (
        9,
        14,
        FeastDetails::new("exaltatio-s-crucis", FeastRank::GreaterDouble).make_secondary(),
    ),
    (
        9,
        15,
        FeastDetails::new(
            "septem-dolorum-bmv-mense-sept",
            FeastRank::DoubleSecondClass,
        )
        .with_person(Person::OurLady)
        .make_secondary(),
    ),
    (
        9,
        15,
        FeastDetails::new("s-nicomedis-m", FeastRank::Commemoration),
    ),
    (
        9,
        16,
        FeastDetails::new("ss-cornelii-p-et-cypriani-e-mm", FeastRank::Semidouble),
    ),
    (
        9,
        16,
        FeastDetails::new(
            "ss-euphemiae-luciae-et-geminiani-mm",
            FeastRank::Commemoration,
        ),
    ),
    (
        9,
        17,
        FeastDetails::new("impressio-s-stigmatum-s-francisci-c", FeastRank::Double)
            .make_secondary(),
    ),
    (
        9,
        18,
        FeastDetails::new("s-josephi-a-cupertino-c", FeastRank::Double),
    ),
    (
        9,
        19,
        FeastDetails::new("ss-januarii-e-et-soc-mm", FeastRank::Double),
    ),
    (
        9,
        20,
        FeastDetails::new("ss-eustachii-et-soc-mm", FeastRank::Double),
    ),
    (
        9,
        21,
        FeastDetails::new("s-matthaei-ap-ev", FeastRank::DoubleSecondClass)
            .with_person(Person::Apostle)
            .with_vigil(VigilRank::Common)
            .make_feriatum(),
    ),
    (
        9,
        22,
        FeastDetails::new("s-thomae-de-villanova-ec", FeastRank::Double),
    ),
    (
        9,
        22,
        FeastDetails::new("ss-mauritii-et-soc-mm", FeastRank::Commemoration),
    ),
    (9, 23, FeastDetails::new("s-lini-pm", FeastRank::Semidouble)),
    (
        9,
        23,
        FeastDetails::new("s-theclae-vm", FeastRank::Commemoration),
    ),
    (
        9,
        24,
        FeastDetails::new("bmv-de-mercede", FeastRank::GreaterDouble)
            .with_person(Person::OurLady)
            .make_secondary(),
    ),
    (
        9,
        26,
        FeastDetails::new("ss-cypriani-et-justinae-v-mm", FeastRank::Simple),
    ),
    (
        9,
        27,
        FeastDetails::new("ss-cosmae-et-damiani-mm", FeastRank::Semidouble),
    ),
    (
        9,
        28,
        FeastDetails::new("s-wenceslai-ducis-m", FeastRank::Semidouble),
    ),
    (
        9,
        29,
        FeastDetails::new(
            "dedicatio-s-michaelis-archangeli",
            FeastRank::DoubleFirstClass,
        )
        .with_person(Person::Angel)
        .make_feriatum(),
    ),
    (
        9,
        30,
        FeastDetails::new("s-hieronymi-cd", FeastRank::Double),
    ),
    (10, 1, FeastDetails::new("s-remigii-ec", FeastRank::Simple)),
    (
        10,
        2,
        FeastDetails::new("ss-angelorum-custodum", FeastRank::GreaterDouble)
            .with_person(Person::Angel),
    ),
    (
        10,
        3,
        FeastDetails::new("s-teresiae-a-jesu-infante-v", FeastRank::Double),
    ),
    (
        10,
        4,
        FeastDetails::new("s-francisci-c", FeastRank::GreaterDouble),
    ),
    (
        10,
        5,
        FeastDetails::new("ss-placidi-et-soc-mm", FeastRank::Simple),
    ),
    (10, 6, FeastDetails::new("s-brunonis-c", FeastRank::Double)),
    (
        10,
        7,
        FeastDetails::new("ss-rosarii-bmv", FeastRank::DoubleSecondClass)
            .with_person(Person::OurLady)
            .make_secondary(),
    ),
    (
        10,
        7,
        FeastDetails::new("s-marci-i-pc", FeastRank::Commemoration),
    ),
    (
        10,
        7,
        FeastDetails::new("ss-sergii-et-soc-mm", FeastRank::Commemoration),
    ),
    (
        10,
        8,
        FeastDetails::new("s-birgittae-vid", FeastRank::Double),
    ),
    (
        10,
        9,
        FeastDetails::new("ss-dionysii-e-et-soc-mm", FeastRank::Semidouble),
    ),
    (
        10,
        10,
        FeastDetails::new("s-francisci-borgiae-c", FeastRank::Semidouble),
    ),
    (
        10,
        11,
        FeastDetails::new("maternitatis-bmv", FeastRank::DoubleSecondClass)
            .with_person(Person::OurLady)
            .make_secondary(),
    ),
    (
        10,
        13,
        FeastDetails::new("s-eduardi-regis-c", FeastRank::Semidouble),
    ),
    (
        10,
        14,
        FeastDetails::new("s-callisti-i-pm", FeastRank::Double),
    ),
    (10, 15, FeastDetails::new("s-teresiae-v", FeastRank::Double)),
    (
        10,
        16,
        FeastDetails::new("s-hedwigis-vid", FeastRank::Semidouble),
    ),
    (
        10,
        17,
        FeastDetails::new("s-margaritae-mariae-alacoque-v", FeastRank::Double),
    ),
    (
        10,
        18,
        FeastDetails::new("s-lucae-ev", FeastRank::DoubleSecondClass)
            .with_person(Person::Evangelist),
    ),
    (
        10,
        19,
        FeastDetails::new("s-petri-de-alcantara-c", FeastRank::Double),
    ),
    (
        10,
        20,
        FeastDetails::new("s-joannis-cantii-c", FeastRank::Double),
    ),
    (
        10,
        21,
        FeastDetails::new("s-hilarionis-abb", FeastRank::Simple),
    ),
    (
        10,
        21,
        FeastDetails::new("ss-ursulae-et-soc-vv-mm", FeastRank::Commemoration),
    ),
    (
        10,
        24,
        FeastDetails::new("s-raphaelis-archangeli", FeastRank::GreaterDouble)
            .with_person(Person::Angel),
    ),
    (
        10,
        25,
        FeastDetails::new("ss-chrysanthi-et-dariae-mm", FeastRank::Simple),
    ),
    (
        10,
        26,
        FeastDetails::new("s-evaristi-pm", FeastRank::Simple),
    ),
    (
        10,
        28,
        FeastDetails::new("ss-simonis-et-judae-app", FeastRank::DoubleSecondClass)
            .with_person(Person::Apostle)
            .with_vigil(VigilRank::Common)
            .make_feriatum(),
    ),
    (
        11,
        1,
        FeastDetails::new("omnium-sanctorum", FeastRank::DoubleFirstClass)
            .make_feriatum()
            .with_vigil(VigilRank::Common)
            .with_octave(OctaveRank::Common)
            .make_feriatum(),
    ),
    (11, 4, FeastDetails::new("s-caroli-ec", FeastRank::Double)),
    (
        11,
        4,
        FeastDetails::new("ss-vitalis-et-agricolae-mm", FeastRank::Commemoration),
    ),
    (
        11,
        8,
        FeastDetails::new("ss-quatuor-coronatorum-mm", FeastRank::Commemoration),
    ),
    (
        11,
        9,
        FeastDetails::new(
            "dedicatio-archibasilicae-ss-salvatoris",
            FeastRank::DoubleSecondClass,
        ),
    ),
    (
        11,
        9,
        FeastDetails::new("s-theodori-m", FeastRank::Commemoration),
    ),
    (
        11,
        10,
        FeastDetails::new("s-andreae-avellini-c", FeastRank::Double),
    ),
    (
        11,
        10,
        FeastDetails::new(
            "ss-tryphonis-respicii-et-nymphae-v-mm",
            FeastRank::Commemoration,
        ),
    ),
    (11, 11, FeastDetails::new("s-martini-ec", FeastRank::Double)),
    (
        11,
        11,
        FeastDetails::new("s-mennae-m", FeastRank::Commemoration),
    ),
    (
        11,
        12,
        FeastDetails::new("s-martini-i-pm", FeastRank::Semidouble),
    ),
    (
        11,
        13,
        FeastDetails::new("s-didaci-c", FeastRank::Semidouble),
    ),
    (
        11,
        14,
        FeastDetails::new("s-josaphat-em", FeastRank::Double),
    ),
    (
        11,
        15,
        FeastDetails::new("s-alberti-magni-ecd", FeastRank::Double),
    ),
    (
        11,
        16,
        FeastDetails::new("s-gertrudis-v", FeastRank::Double),
    ),
    (
        11,
        17,
        FeastDetails::new("s-gregorii-thaumaturgi-ec", FeastRank::Semidouble),
    ),
    (
        11,
        18,
        FeastDetails::new(
            "dedicatio-basilicarum-ss-petri-et-pauli-app",
            FeastRank::GreaterDouble,
        ),
    ),
    (
        11,
        19,
        FeastDetails::new("s-elisabeth-vid", FeastRank::Double),
    ),
    (
        11,
        19,
        FeastDetails::new("s-pontiani-pm", FeastRank::Commemoration),
    ),
    (
        11,
        20,
        FeastDetails::new("s-felicis-de-valois-c", FeastRank::Double),
    ),
    (
        11,
        21,
        FeastDetails::new("praesentatio-bmv", FeastRank::GreaterDouble)
            .with_person(Person::OurLady),
    ),
    (
        11,
        22,
        FeastDetails::new("s-caeciliae-vm", FeastRank::Double),
    ),
    (
        11,
        23,
        FeastDetails::new("s-clementis-i-pm", FeastRank::Double),
    ),
    (
        11,
        23,
        FeastDetails::new("s-felicitatis-m", FeastRank::Commemoration),
    ),
    (
        11,
        24,
        FeastDetails::new("s-joannis-a-cruce-cd", FeastRank::Double),
    ),
    (
        11,
        24,
        FeastDetails::new("s-chrysogoni-m", FeastRank::Commemoration),
    ),
    (
        11,
        25,
        FeastDetails::new("s-catharinae-vm", FeastRank::Double),
    ),
    (
        11,
        26,
        FeastDetails::new("s-silvestri-abb", FeastRank::Double),
    ),
    (
        11,
        26,
        FeastDetails::new("s-petri-alexandrini-em", FeastRank::Commemoration),
    ),
    (
        11,
        29,
        FeastDetails::new("s-saturnini-m", FeastRank::Commemoration),
    ),
    (
        11,
        30,
        FeastDetails::new("s-andreae-ap", FeastRank::DoubleSecondClass)
            .with_person(Person::Apostle)
            .with_vigil(VigilRank::Common)
            .make_feriatum(),
    ),
    (
        12,
        2,
        FeastDetails::new("s-bibianae-vm", FeastRank::Semidouble),
    ),
    (
        12,
        3,
        FeastDetails::new("s-francisci-xaverii-c", FeastRank::GreaterDouble),
    ),
    (
        12,
        4,
        FeastDetails::new("s-petri-chrysologi-ecd", FeastRank::Double),
    ),
    (
        12,
        4,
        FeastDetails::new("s-barbarae-vm", FeastRank::Commemoration),
    ),
    (
        12,
        5,
        FeastDetails::new("s-sabbae-abb", FeastRank::Commemoration),
    ),
    (12, 6, FeastDetails::new("s-nicolai-ec", FeastRank::Double)),
    (
        12,
        7,
        FeastDetails::new("s-ambrosii-ecd", FeastRank::Double),
    ),
    (
        12,
        8,
        FeastDetails::new("conceptio-immaculata-bmv", FeastRank::DoubleFirstClass)
            .with_person(Person::OurLady)
            .with_vigil(VigilRank::Common)
            .with_octave(OctaveRank::Common)
            .make_feriatum(),
    ),
    (
        12,
        10,
        FeastDetails::new("s-melchiadis-pm", FeastRank::Commemoration),
    ),
    (
        12,
        11,
        FeastDetails::new("s-damasi-i-pc", FeastRank::Semidouble),
    ),
    (12, 13, FeastDetails::new("s-luciae-vm", FeastRank::Double)),
    (
        12,
        16,
        FeastDetails::new("s-eusebii-em", FeastRank::Semidouble),
    ),
    (
        12,
        21,
        FeastDetails::new("s-thomae-ap", FeastRank::DoubleSecondClass)
            .with_person(Person::Apostle)
            .with_vigil(VigilRank::Common)
            .make_feriatum(),
    ),
    (
        12,
        25,
        FeastDetails::new("nativitas-dnjc", FeastRank::DoubleFirstClass)
            .with_person(Person::OurLord)
            .with_octave(OctaveRank::ThirdOrder)
            .with_vigil(VigilRank::FirstClass)
            .make_feriatum(),
    ),
    (
        12,
        26,
        FeastDetails::new("s-stephani-protomartyris", FeastRank::DoubleSecondClass)
            .with_octave(OctaveRank::Simple)
            .make_feriatum(),
    ),
    (
        12,
        27,
        FeastDetails::new("s-joannis-ap-ev", FeastRank::DoubleSecondClass)
            .with_person(Person::Apostle)
            .with_octave(OctaveRank::Simple)
            .make_feriatum(),
    ),
    (
        12,
        28,
        FeastDetails::new("ss-innocentium-mm", FeastRank::DoubleSecondClass)
            .with_octave(OctaveRank::Simple)
            .make_feriatum(),
    ),
    (12, 29, FeastDetails::new("s-thomas-em", FeastRank::Double)),
    (
        12,
        31,
        FeastDetails::new("s-silvestri-i-pc", FeastRank::Double),
    ),
];
