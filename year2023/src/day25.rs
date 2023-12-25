use std::collections::{BTreeSet, VecDeque};

use super::*;

use itertools::Itertools;
use petgraph::*;

fn calc_flow(
    graph: &Graph<&str, (), Undirected>,
    source: graph::NodeIndex,
    sink: graph::NodeIndex,
) -> u32 {
    assert_ne!(source, sink);

    fn find_aug_path(
        graph: &Graph<&str, (), Undirected>,
        source: graph::NodeIndex,
        sink: graph::NodeIndex,
        flow: &mut BTreeSet<(graph::NodeIndex, graph::NodeIndex)>,
    ) -> bool {
        let mut pred = vec![None; graph.node_count()];

        let mut q = VecDeque::new();
        q.push_back(source);

        while let Some(node) = q.pop_front() {
            if node == sink {
                break;
            }
            for neighbor in graph.neighbors(node) {
                if neighbor != source
                    && pred[neighbor.index()].is_none()
                    && !flow.contains(&(node, neighbor))
                {
                    pred[neighbor.index()] = Some(node);
                    q.push_back(neighbor);
                }
            }
        }

        if pred[sink.index()].is_none() {
            return false;
        }

        for (b, a) in std::iter::successors(Some(sink), |x| pred[x.index()]).tuple_windows() {
            if flow.contains(&(b, a)) {
                flow.remove(&(b, a));
            } else {
                flow.insert((a, b));
            }
        }

        true
    }

    let mut flow = BTreeSet::new();

    let mut counter = 0;
    while find_aug_path(graph, source, sink, &mut flow) {
        counter += 1;
    }

    counter
}

pub struct Day25;
impl Solution for Day25 {
    type Input<'a> = Graph<&'a str, (), Undirected>;

    fn parse<'a>(content: &'a str) -> Self::Input<'a> {
        parsing::parse(content).unwrap().1
    }

    fn part_a<'a>(input: &Self::Input<'a>) -> String {
        let ref_node = input.node_indices().next().unwrap();

        let group = input
            .node_indices()
            .filter(|&x| x == ref_node || calc_flow(input, ref_node, x) > 3)
            .collect_vec();

        format!("{}", group.len() * (input.node_count() - group.len()))
    }

    fn part_b<'a>(_input: &Self::Input<'a>) -> String {
        "Push the button!".to_owned()
    }
}

mod parsing {
    use std::collections::HashMap;

    use nom::{
        character::complete::{alpha1, char, line_ending, space0, space1},
        multi::separated_list0,
        sequence::{separated_pair, tuple},
        *,
    };

    use petgraph::*;

    fn line(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
        separated_pair(
            alpha1,
            tuple((space0, char(':'), space0)),
            separated_list0(space1, alpha1),
        )(input)
    }

    pub(super) fn parse(input: &str) -> IResult<&str, Graph<&str, (), Undirected>> {
        separated_list0(line_ending, line)
            .map(|lines| {
                lines
                    .into_iter()
                    .fold(
                        (Graph::new_undirected(), HashMap::new()),
                        |(mut graph, mut names), (node, neighbors)| {
                            let g_node = *names.entry(node).or_insert_with(|| graph.add_node(node));

                            for neighbor in neighbors {
                                let g_neighbor = *names
                                    .entry(neighbor)
                                    .or_insert_with(|| graph.add_node(neighbor));
                                graph.update_edge(g_node, g_neighbor, ());
                            }

                            (graph, names)
                        },
                    )
                    .0
            })
            .parse(input)
    }
}

gen_test!(
    a,
    Day25,
    r"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
",
    "54"
);
