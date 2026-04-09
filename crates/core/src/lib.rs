

/// Définis les types de messages transférés entres les éléments du projet
#[derive(Clone)]
pub enum TypeMessage{
    Connexion((String,AlgoType)),
    Tab(Vec<u64>),
    Resultmessage((u128,u64)),
    Stats(Vec<StatsMessage>)
}

/// Définis la structure des statistique attendu par le dashboard
#[derive(Clone)]
pub struct StatsMessage {
    pub name : String,
    pub type_algo : AlgoType,
    pub result : (u128,u64)
}

/// Définis différent type de tri, nottament le tri implémenter par Rust avec sort_by et a.cmp(b)
#[derive(Clone)]
pub enum AlgoType{
    Bubblesort,
    Insertionsort,
    Mergesort,

}

/// définie le comportement d'affichage
impl std::fmt::Display for AlgoType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AlgoType::Bubblesort   => write!(f, "BUBBLE"),
            AlgoType::Insertionsort    => write!(f, "INSERTION"),
            AlgoType::Mergesort    => write!(f, "MERGE"),
        }
    }
}

pub struct BubbleSorter;
pub struct InsertionSorter;
pub struct MergeSorter;

pub trait Sorter {
    fn sorting(&self, tab : &mut Vec<u64>, name : &String) -> StatsMessage;
}

impl Sorter for BubbleSorter {
    fn sorting(&self, vec : &mut Vec<u64>, name : &String) -> StatsMessage{
        let start = time();
        let n  = vec.len();
        let mut count = 0;
        for _i in 0..n-1 {
            for j in 0..n-_i-1 {
                count += 1;
                if vec[j] > vec[j+1]{
                    swap(vec,j,j+1);
                }
            }
        }
        let end = time() - start;
        StatsMessage{name : name.clone(), type_algo : AlgoType::Bubblesort, result : (end as u128,count as u64)}
    }

    
}

impl Sorter for InsertionSorter{
    fn sorting(&self, vec : &mut Vec<u64>, name : &String) -> StatsMessage{
        let start = time();
        let n  = vec.len();
        let mut count = 0;
        for i in 1..n {
            let key = vec[i];
            let mut j = i-1;
            while vec[j] > key {
                count += 1;
                vec[j+1] = vec[j];
                match j.checked_sub(1) {
                    Some(new_j) => j = new_j,
                    None => { vec[0] = key; break; }
                }
            }
            count += 1;
            vec[j+1]=key;
        }
        let end = time() - start;
        StatsMessage{name : name.clone(), type_algo : AlgoType::Insertionsort, result : (end as u128,count as u64)}
    }
}

impl Sorter for MergeSorter{
    fn sorting(&self, vec : &mut Vec<u64>, name : &String) -> StatsMessage{
        let start = time();
        let l: usize = 0;
        let r:usize = vec.len()-1;
        let mut count: u64 = 0;
        merge_sort(vec,l,r,&mut count);
        let end = time() - start;
        StatsMessage{name : name.clone(), type_algo : AlgoType::Mergesort, result : (end as u128,count as u64)}

    }
}


fn merge(vec : &mut Vec<u64>, l : usize, m: usize, r : usize, count : &mut u64){
    let v_left = vec[l..=m].to_vec();
    let v_right = vec[m+1..=r].to_vec();
    let mut i=0;let mut j=0;let mut k=l;
    while i<v_left.len() && j<v_right.len() {
        *count += 1;
        if v_left[i]<=v_right[j]{
            vec[k]=v_left[i];
            i=i+1;
            k=k+1;
        }
        else {
            vec[k]=v_right[j];
            j=j+1;
            k=k+1;
        }
    }
    while i<v_left.len() {
        vec[k]=v_left[i];
        i=i+1;
        k=k+1;
    }
    while j<v_right.len() {
        vec[k]=v_right[j];
        j=j+1;
        k=k+1;
    }
}

fn merge_sort(vec : &mut Vec<u64>, l: usize, r : usize, count : &mut u64){
    if l>=r {return};
    let m = (l+r)/2;
    merge_sort(vec, l, m, count);
    merge_sort(vec, m+1, r, count);
    merge(vec,l,m,r, count);

}

fn swap(vec: &mut Vec<u64>, nb1 : usize, nb2 : usize){
        let temp = vec[nb1];
        vec[nb1] = vec[nb2];
        vec[nb2] = temp

    }

fn time() -> u128{
    return std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros();
}



/// Transforme le string reçu en un TypeMessage lissible et utilisable 
pub fn tcp_to_typemessage(log : String) -> Option<TypeMessage>{
    let groups = log.split('|').collect::<Vec<&str>>();
    match groups[0] {
        "CONNECT"   => {
            match groups[2]{
               "BUBBLE" => Some(TypeMessage::Connexion((groups[1].to_string(),AlgoType::Bubblesort))),
               "INSERTION" => Some(TypeMessage::Connexion((groups[1].to_string(),AlgoType::Insertionsort))),
               "MERGE" => Some(TypeMessage::Connexion((groups[1].to_string(),AlgoType::Mergesort))),
               _ => Some(TypeMessage::Connexion((groups[1].to_string(),AlgoType::Bubblesort))),
            }
        },
        "TAB"       => {
            let mut vec = Vec::new();
            for i in 1..groups.len()-1{
                let value : u64 = groups[i].parse().unwrap();
                vec.push(value);
            }
            Some(TypeMessage::Tab(vec))
        },
        "RESULT"    => {
            let time : u128 = groups[1].parse().unwrap();
            let count : u64 = groups[2].trim().parse().unwrap();
            Some(TypeMessage::Resultmessage((time,count)))
        },
        "STAT"      => {
            let mut vec : Vec<StatsMessage> = Vec::new();
            let size = (groups.len() - 1)/4;
            for i in 0..size{ 
                let algo = match groups[2+i*4] {
                    "BUBBLE" => AlgoType::Bubblesort,
                    "INSERTION" => AlgoType::Insertionsort,
                    "MERGE" => AlgoType::Mergesort,
                    _ => AlgoType::Mergesort,
                    };
                let time : u128 = groups[3+i*4].parse().unwrap();
                let count : u64 = groups[4+i*4].trim().parse().unwrap();
                vec.push(StatsMessage { name: groups[1+i*4].to_string(), type_algo: algo, result: (time,count) });
            }
            Some(TypeMessage::Stats(vec))
        },
        _           => None 
    }
}

pub fn typemessage_to_tcp(msg : &TypeMessage) -> Option<String>{
    match msg {
        TypeMessage::Connexion((a,b)) => {
            match b{
                AlgoType::Bubblesort => Some(format!("CONNECT|{}|BUBBLE\n",a)),
                AlgoType::Insertionsort => Some(format!("CONNECT|{}|INSERTION\n",a)),
                AlgoType::Mergesort => Some(format!("CONNECT|{}|MERGE\n",a)),
            }
        },
        TypeMessage::Tab(vec)   => {
            let mut texte = format!("TAB");
            for i in vec {
                texte = format!("{}|{}",texte,i);
            }
            Some( format!("{}\n",texte))
        },
        TypeMessage::Resultmessage((a,b)) => {
            Some(format!("RESULT|{}|{}\n",a,b))
        },
        TypeMessage::Stats(vec) => {
            let mut texte = format!("STAT");
            for stat_msg in vec {
                match stat_msg.type_algo{
                    AlgoType::Bubblesort => {texte = format!("{}|{}|BUBBLE|{}|{}",texte,stat_msg.name,stat_msg.result.0,stat_msg.result.1);},
                    AlgoType::Insertionsort => {texte = format!("{}|{}|INSERTION|{}|{}",texte,stat_msg.name,stat_msg.result.0,stat_msg.result.1);},
                    AlgoType::Mergesort => {texte = format!("{}|{}|MERGE|{}|{}",texte,stat_msg.name,stat_msg.result.0,stat_msg.result.1);}
                }   
            }
            Some( format!("{}\n",texte))
        },
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
