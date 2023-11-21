use super::*;

const SEPTEM_DOLORUM_BMV_TEMP_PASS: Office =
    Office::feast("septem-dolorum-bmv-temp-pass", FeastRank::GreaterDouble)
        .with_person(Person::OurLady)
        .done();

const SOLEMNITY_ST_JOSEPH: Office =
    Office::feast("solemnitas-s-joseph", FeastRank::DoubleFirstClass)
        .with_person(Person::Joseph)
        .with_octave(OctaveRank::Common)
        .done();

impl Calendar1939 {
    pub fn add_moveable_feasts<'a>(self, ch: CalendarHelper, days: &mut Vec<Vec<Office<'a>>>) {
        days[ch.easter - 9].push(SEPTEM_DOLORUM_BMV_TEMP_PASS);
        days[ch.easter + 17].push(SOLEMNITY_ST_JOSEPH);
    }
}

pub const CALENDAR_OF_SAINTS: [CalendarEntry; 114] = [
    (
        1,
        5,
        FeastDetails::new("s-telesphori-pm", FeastRank::Commemoration),
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
        FeastDetails::new("s-apolloniae-vm", FeastRank::Double),
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
            .with_person(Person::OurLady),
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
            .with_vigil(VigilRank::Common),
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
            .with_person(Person::Joseph),
    ),
    (
        3,
        21,
        FeastDetails::new("s-benedicti-abb", FeastRank::GreaterDouble),
    ),
    (
        3,
        24,
        FeastDetails::new("s-gabrielis-archang", FeastRank::GreaterDouble),
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
        FeastDetails::new("inventio-s-crucis", FeastRank::DoubleSecondClass),
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
            .with_person(Person::Apostle),
    ),
    (5, 7, FeastDetails::new("s-stanislai-em", FeastRank::Double)),
    (
        5,
        8,
        FeastDetails::new("apparitio-s-michaelis-archang", FeastRank::GreaterDouble)
            .with_person(Person::Angels),
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
        11,
        1,
        FeastDetails::new("omnium-sanctorum", FeastRank::DoubleFirstClass)
            .make_feriatum()
            .with_vigil(VigilRank::Common)
            .with_octave(OctaveRank::Common),
    ),
];
