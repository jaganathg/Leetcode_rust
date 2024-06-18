
fn dna_strand(dna: &str) -> String {
    dna.chars().map(|c| match c {
        'A' => 'T',
        'T' => 'A',
        'C' => 'G',
        'G' => 'C',
        _ => c,
    }).collect()
}

fn main()
{
    let dna = "ATTGC";
    let result = dna_strand(dna);
    println!("{:?}", result);
    let dna = "GTAT";
    let result = dna_strand(dna);
    println!("{:?}", result);
    let dna = "CATA";
    let result = dna_strand(dna);
    println!("{:?}", result);
    let dna = "GCGC";
    let result = dna_strand(dna);
    println!("{:?}", result);
    let dna = "ATAT";
    let result = dna_strand(dna);
    println!("{:?}", result);
    let dna = "CGCG";
    let result = dna_strand(dna);
    println!("{:?}", result);
    let dna = "TATA";
    let result = dna_strand(dna);
    println!("{:?}", result);
    let dna = "ACGT";
    let result = dna_strand(dna);
    println!("{:?}", result);
    let dna = "GCTA";
    let result = dna_strand(dna);
    println!("{:?}", result);
    let dna = "TACG";
    let result = dna_strand(dna);
    println!("{:?}", result);
    let dna = "CGTA";
    let result = dna_strand(dna);
    println!("{:?}", result);
    let dna = "ATCG";
    let result = dna_strand(dna);
    println!("{:?}", result);
    let dna = "GATC";
    let result = dna_strand(dna);
    println!("{:?}", result);
    let dna = "TACG";
    let result = dna_strand(dna);
    println!("{:?}", result);
    let dna = "CGTA";
    let result = dna_strand(dna);
    println!("{:?}", result);
    let dna = "ATCG";
    let result = dna_strand(dna);
    println!("{:?}", result);
    let dna = "GATC";
    let result = dna_strand(dna);
    println!("{:?}", result);
    let dna = "TACG";
    let result = dna_strand(dna);
    println!("{:?}", result);
    let dna = "CGTA";
    let result = dna_strand(dna);
    println!("{:?}", result);
    
}