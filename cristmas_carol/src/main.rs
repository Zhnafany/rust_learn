
const VERSE: [&str; 12] = [
"My True Love, the Partridge in a Pear Tree (Jesus Christ is my true love). In ancient times a partridge was often used as symbol of a divine and sacred king.",
"Two turtle doves are the Old and New Testaments of the Bible. The doves symbolize peace.",
"The three French Hens are Faith, Hope and Love. These are the three gifts of the Holy Spirit.",
"The four calling birds are the four Gospels - Matthew, Mark, Luke and John.",
"The five golden rings describe the first five books of the Old Testament.",
"The six geese a laying stood for the first six days of creation.",
"The seven swans a swimming represented the the sevenfold gifts of the Holy Spirit. These are Prophesy, Serving, Teaching, Exhortation, Contribution, Leadership and Mercy.",
"The eight maids a milking are the eight Beatitudes. These are Jesus' teachings of happiness.",
"Nine ladies dancing are the nine fruits of the Holy Spirit. These are Charity, Joy, Peace, Patience, Goodness, Mildness, Fidelity, Modesty and Continency.",
"The ten lords a leaping are the Ten Commandments.",
"The eleven pipers piping represent the eleven faithful apostles.",
"The twelve drummers drumming represent the twelve points of belief in The Apostles' Creed."
];

const DAYS: [&str; 12] = [ "first", "second ", "third", "fourth ", "fifth", "sixth",
"seventh", "eighth ", "ninth", "tenth", "elevent", "twelfth"];

fn main() { for number in 1..12 {
        println!("{}. The {} day of Christmas - {}", number,
                 DAYS[number - 1], VERSE[number - 1]);
    }
}
