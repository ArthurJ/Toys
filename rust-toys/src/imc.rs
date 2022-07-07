#[derive(Debug)]
pub enum IMC{
    MagrezaGrave,
    MagrezaModerada,
    MagrezaLeve,
    Saudavel,
    Sobrepeso,
    Obesidade,
    ObesidadeSevera,
    ObesidadeMorbida
}

pub fn calc_imc(altura: f64, peso: f64)-> IMC {
    let imc = peso / (altura*altura);
    match imc {
        _ if (imc<16.0) => IMC::MagrezaGrave,
        _ if (imc<17.0) => IMC::MagrezaModerada,
        _ if (imc<18.5) => IMC::MagrezaLeve,
        _ if (imc<25.0) => IMC::Saudavel,
        _ if (imc<30.0) => IMC::Sobrepeso,
        _ if (imc<35.0) => IMC::Obesidade,
        _ if (imc<40.0) => IMC::ObesidadeSevera,
        _ => IMC::ObesidadeMorbida
    }}