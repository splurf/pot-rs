use pot_rs::Machine;

fn main() {
    // Since the following system is dependent on the `From` and `Into` correspondents, heavy explicit type annotation is required
    let machine: Machine<(), ()> = |_| -> () {}.into();

    //  Having the machine perform with it's given instructions (specified parameter)
    machine.perform(())
}
