extern crate conv;

use self::conv::prelude::*;
use neat::genome::Genome;
use neat::organism::Organism;


#[derive(Debug, Clone)]
pub struct Specie{
    representative: Genome,
    pub organisms: Vec<Organism>
}

impl Specie{
    pub fn new(genome: Genome) -> Specie{
        Specie{ organisms: vec![], representative: genome }
    }

    pub fn add(&mut self, organism: Organism){
        self.organisms.push(organism);
    }

    pub fn match_genome(&self, organism: &Organism) -> bool{
        self.representative.is_same_specie(&organism.genome)
    }

    pub fn average_fitness(&self) -> f64{
        let total_fitness = self.organisms.iter().fold(0f64, |total, organism| total + organism.fitness);
        total_fitness / self.organisms.len().value_as::<f64>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use neat::*;

    #[test]
    fn specie_should_return_correct_average_fitness(){
        let mut specie = Specie::new(Genome::new(1, 1));
        let mut organism1 = Organism::new(Genome::new(1, 1));
        organism1.fitness = 10f64;

        let mut organism2 = Organism::new(Genome::new(1, 1));
        organism2.fitness = 15f64;

        let mut organism3 = Organism::new(Genome::new(1, 1));
        organism3.fitness = 20f64;

        specie.add(organism1);
        specie.add(organism2);
        specie.add(organism3);

        assert!(specie.average_fitness() == 15f64);
    }
}
