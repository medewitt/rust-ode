use ode_solvers::dopri5::*;
use ode_solvers::*;
use std::{f64::consts::PI, fs::File, io::BufWriter, io::Write, path::Path};
use rand_distr::{Distribution, Poisson, Normal, NormalError};
use rand::thread_rng;


type State = Vector3<f64>;
type Time = f64;

struct SIRParams {
    beta: f64,
    gamma: f64,
    kappa: f64,
    mu: f64,
  }

  

  impl ode_solvers::System<State> for SIRParams {
    // Equations of the system
    fn system(&self, _t: Time, y: &State, dy: &mut State) {
        let pop_n = y[0] + y[1] + y[2];
        // Some randomness as a pseudo pde though it isn't
        //let  poi = Poisson::new(1.0).unwrap();
        //let  v = poi.sample(&mut rand::thread_rng());
        
        dy[0] = - self.beta * y[0]*y[1]/pop_n +self.kappa * y[2] + self.mu * pop_n - y[0] * self.mu;
        dy[1] = self.beta * y[0]*y[1]/pop_n - self.gamma * y[1] -y[1] * self.mu;
        dy[2] = self.gamma * y[1] - self.kappa * y[2] - y[2] * self.mu;
        //println!("{v}\n")

    }
}

// From the ode_solver github
pub fn save(times: &Vec<Time>, states: &Vec<State>, filename: &Path) {
    // Create or open file
    let file = match File::create(filename) {
        Err(e) => {
            println!("Could not open file. Error: {:?}", e);
            return;
        }
        Ok(buf) => buf,
    };
    let mut buf = BufWriter::new(file);

    // Write time and state vector in a csv format
    for (i, state) in states.iter().enumerate() {
        buf.write_fmt(format_args!("{}", times[i])).unwrap();
        for val in state.iter() {
            buf.write_fmt(format_args!(", {}", val)).unwrap();
        }
        buf.write_fmt(format_args!("\n")).unwrap();
    }
    if let Err(e) = buf.flush() {
        println!("Could not write to file. Error: {:?}", e);
    }
}

fn main() {
    let system = SIRParams {beta: 0.25, gamma: 0.10, kappa: 0.0013, mu: 0.0005};

    let y0 = State::new(999999.0, 1.0, 0.0);

    let mut stepper = Dopri5::new(system, 0.0,3650.0, 1.0, y0, 1.0e-4, 1.0e-4);
    let res = stepper.integrate();

    // Handle result
    match res {
        Ok(stats) => {
            stats.print();
            // Do something with the results...
            let path = Path::new("./outputs/output.dat");
            save(stepper.x_out(), stepper.y_out(), path);  
            println!("Results saved in: {:?}", path);
        },
        Err(_) => println!("An error occured."),
    }
}