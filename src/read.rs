extern crate csv;
extern crate cached;

pub mod read
{
    fn example()
    {
        let mut rdr = csv::Reader::from_reader(io::stdin());
        for result in rdr.deserialize()
        {
            let record: Record = result?;
            println!("{:?}", record);
        }
    }

    pub fn exec() {
        example();
    }
}