#[derive(Clone, Debug)]
struct SubatomicParticle {
    name: String,
    quarks: Vec<Quark>,
    beta_plus_product: bool,
    mass: f64, // MeV
    stable_in_vacuum: bool,
    is_lepton: bool,
    charge: i8,
}

#[derive(Clone, Debug)]
enum Quark {
    Up,
    Down,
    Charm,
    Strange,
    Top,
    Bottom,
    AntiUp,
    AntiDown,
    AntiCharm,
    AntiStrange,
    AntiTop,
    AntiBottom,
}

impl Quark {
    fn get_charge(&self) -> i8 {
        match self {
            Quark::Up => 2,
            Quark::Down => -1,
            Quark::Charm => 2,
            Quark::Strange => -1,
            Quark::Top => 2,
            Quark::Bottom => -1,
            Quark::AntiUp => -2,
            Quark::AntiDown => 1,
            Quark::AntiCharm => -2,
            Quark::AntiStrange => 1,
            Quark::AntiTop => -2,
            Quark::AntiBottom => 1,
        }
    }
}

fn main() {
   let electron = SubatomicParticle {
        name: "Electron".to_string(),
        quarks: vec![],
        beta_plus_product: false,
        mass: 0.511,
        stable_in_vacuum: true,
        is_lepton: true,
        charge: -1,
   };

   let positron = SubatomicParticle {
        name: "Positron".to_string(),
        quarks: vec![],
        beta_plus_product: true,
        mass: 0.511,
        stable_in_vacuum: true,
        is_lepton: true,
        charge: 1,
   };

   let anti_muon = SubatomicParticle {
        name: "Anti-Muon".to_string(),
        quarks: vec![],
        beta_plus_product: false,
        mass: 105.658,
        stable_in_vacuum: false,
        is_lepton: true,
        charge: 1,
   };

   let muon = SubatomicParticle {
        name: "Muon".to_string(),
        quarks: vec![],
        beta_plus_product: false,
        mass: 105.658,
        stable_in_vacuum: false,
        is_lepton: true,
        charge: -1,
   };

   let pi_plus = SubatomicParticle {
        name: "Pi+".to_string(),
        quarks: vec![
            Quark::Up,
            Quark::AntiDown,
        ],
        beta_plus_product: false,
        mass: 139.570,
        stable_in_vacuum: false,
        is_lepton: false,
        charge: 1,
   };

   let pi_minus = SubatomicParticle {
        name: "Pi-".to_string(),
        quarks: vec![
            Quark::Down,
            Quark::AntiUp,
        ],
        beta_plus_product: false,
        mass: 139.570,
        stable_in_vacuum: false,
        is_lepton: false,
        charge: -1,
   };

   let k_0 = SubatomicParticle {
        name: "K0".to_string(),
        quarks: vec![
            Quark::Down,
            Quark::AntiStrange,
        ],
        beta_plus_product: false,
        mass: 497.648,
        stable_in_vacuum: false,
        is_lepton: false,
        charge: 0,
   };

   let anti_k_0 = SubatomicParticle {
        name: "Anti-K0".to_string(),
        quarks: vec![
            Quark::Strange,
            Quark::AntiDown,
        ],
        beta_plus_product: false,
        mass: 497.648,
        stable_in_vacuum: false,
        is_lepton: false,
        charge: 0,
   };

   let k_minus = SubatomicParticle {
        name: "K-".to_string(),
        quarks: vec![
            Quark::Strange,
            Quark::AntiUp,
        ],
        beta_plus_product: false,
        mass: 493.677,
        stable_in_vacuum: false,
        is_lepton: false,
        charge: -1,
   };

    let k_plus = SubatomicParticle {
          name: "K+".to_string(),
          quarks: vec![
                Quark::Up,
                Quark::AntiStrange,
          ],
          beta_plus_product: false,
          mass: 493.677,
          stable_in_vacuum: false,
          is_lepton: false,
          charge: 1,
    };

    let proton = SubatomicParticle {
          name: "Proton".to_string(),
          quarks: vec![
                Quark::Up,
                Quark::Up,
                Quark::Down,
          ],
          beta_plus_product: false,
          mass: 938.272,
          stable_in_vacuum: true,
          is_lepton: false,
          charge: 1,
    };

    let anti_proton = SubatomicParticle {
          name: "Anti-Proton".to_string(),
          quarks: vec![
                Quark::AntiUp,
                Quark::AntiUp,
                Quark::AntiDown,
          ],
          beta_plus_product: false,
          mass: 938.272,
          stable_in_vacuum: true,
          is_lepton: false,
          charge: -1,
    };

    let neutron = SubatomicParticle {
          name: "Neutron".to_string(),
          quarks: vec![
                Quark::Up,
                Quark::Down,
                Quark::Down,
          ],
          beta_plus_product: true,
          mass: 939.565,
          stable_in_vacuum: true,
          is_lepton: false,
          charge: 0,
    };

    let anti_neutron = SubatomicParticle {
          name: "Anti-Neutron".to_string(),
          quarks: vec![
                Quark::AntiUp,
                Quark::AntiDown,
                Quark::AntiDown,
          ],
          beta_plus_product: false,
          mass: 939.565,
          stable_in_vacuum: true,
          is_lepton: false,
          charge: 0,
    };

    let tau = SubatomicParticle {
          name: "Tau".to_string(),
          quarks: vec![],
          beta_plus_product: false,
          mass: 1776.86,
          stable_in_vacuum: false,
          is_lepton: true,
          charge: -1,
    };

    let subatomic_particles = vec![
        electron.clone(),
        positron.clone(),
        anti_muon.clone(),
        muon.clone(),
        pi_plus.clone(),
        pi_minus.clone(),
        k_0.clone(),
        anti_k_0.clone(),
        k_minus.clone(),
        k_plus.clone(),
        proton.clone(),
        anti_proton.clone(),
        neutron.clone(),
        anti_neutron.clone(),
        tau.clone(),
    ];

    println!("Checking condition A works");
    let connected_leptons: [SubatomicParticle; 5] = [
        anti_muon.clone(),
        electron.clone(),
        anti_muon.clone(),
        proton.clone(),
        neutron.clone(),
    ];

    assert_eq!(check_condition_e(&connected_leptons), true);

    // Generate all permutations of 5 subatomic particles with no duplicate particles in a set
    for a in &subatomic_particles {
        for b in &subatomic_particles {
            for c in &subatomic_particles {
                for d in &subatomic_particles {
                    for e in &subatomic_particles {
                        let permutation = [a.clone(), b.clone(), c.clone(), d.clone(), e.clone()];

                        // Check if there are any duplicates
                        let mut has_duplicates = false;
                        for (i, particle) in permutation.iter().enumerate() {
                            for (j, other_particle) in permutation.iter().enumerate() {
                                if i != j && particle.name == other_particle.name {
                                    has_duplicates = true;
                                }
                            }
                        }

                        if has_duplicates {
                            continue;
                        }

                        // Check condition A
                        if !check_condition_a(&permutation) {
                            continue;
                        }

                        // Check condition B
                        if !check_condition_b(&permutation) {
                            continue;
                        }

                        // Check condition C
                        if !check_condition_c(&permutation) {
                            continue;
                        }

                        // Check condition D
                        if !check_condition_d(&permutation) {
                            continue;
                        }

                        // Check condition E
                        if !check_condition_e(&permutation) {
                            continue;
                        }

                        let pad_amount = 15;
                        let output_string = format!("{} {} {} {} {}", right_pad(&a.name, pad_amount), right_pad(&b.name, pad_amount), right_pad(&c.name, pad_amount), right_pad(&d.name, pad_amount), right_pad(&e.name, pad_amount));

                        println!("{}", output_string);
                    }
                }
            }
        }
    }

    println!("Permutations done!");
}

fn right_pad(string: &str, length: usize) -> String {
    let mut string = string.to_string();

    while string.len() < length {
        string.push(' ');
    }

    string
}

fn check_condition_a(particles: &[SubatomicParticle; 5]) -> bool {
    //B,c,d,e have twice as many down as anti down, and twice as many up as anti up
    let mut down_count = 0;
    let mut anti_down_count = 0;
    let mut up_count = 0;
    let mut anti_up_count = 0;

    // Trim to the last 4 particles
    let particles = &particles[1..5];

    // Iterate
    for particle in particles {
        for quark in &particle.quarks {
            match quark {
                Quark::Down => down_count += 1,
                Quark::AntiDown => anti_down_count += 1,
                Quark::Up => up_count += 1,
                Quark::AntiUp => anti_up_count += 1,
                _ => (),
            }
        }
    }

    // Check condition
   down_count == anti_down_count * 2 && up_count == anti_up_count * 2
}

fn check_condition_b(particles: &[SubatomicParticle; 5]) -> bool {
    // B is a product of beta-plus decay
    let b = &particles[1];

    b.beta_plus_product
}

fn check_condition_c(particles: &[SubatomicParticle; 5]) -> bool {
    // a,b,d,e have a total charge of 0
    particles[0].charge + particles[1].charge + particles[3].charge + particles[4].charge == 0
}

fn check_condition_d(particles: &[SubatomicParticle; 5]) -> bool {
    // D has a higher mass than C and E
    let c = &particles[2];
    let d = &particles[3];
    let e = &particles[4];

    //println!("{} {} {}", c.mass, d.mass, e.mass);

    d.mass > c.mass && d.mass > e.mass
}

fn check_condition_e(particles: &[SubatomicParticle; 5]) -> bool {
    // Only one lepton is stable in a vacuum
    let mut stable_lepton_count = 0;

    for particle in particles {
        if particle.is_lepton && particle.stable_in_vacuum {
            stable_lepton_count += 1;
        }
    }

    if stable_lepton_count != 1 {
        return false;
    }

    // The leptons are all sitting next to each other
    // Trim to the first lepton
    let mut is_lepton_group = false;
    let mut lepton_group_count = 0;

    // Iterate
    for particle in particles {
        if particle.is_lepton {
            if !is_lepton_group {
                is_lepton_group = true;
                lepton_group_count += 1;
            }
        } else {
            is_lepton_group = false;
        }
    }

    lepton_group_count < 2
}