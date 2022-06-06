
    fn dfs_helper(&mut self, graph: &mut Graph, node_index: NodeIndex) {
        self.add_step(AlgorithmStep::Node(NodeStep::new(
            node_index,
            NodeState::Queued,
        )));

        if let Some(node) = graph.node_weight_mut(node_index) {
            node.set_state(NodeState::Queued)
        }

        let mut walker = graph
            .neighbors_directed(node_index, Direction::Outgoing)
            .detach();

        while let Some((edge_idx, other_node_idx)) = walker.next(graph) {
            if let Some(other_state) = graph
                .node_weight(other_node_idx)
                .map(|node| node.get_state())
            {
                if matches!(other_state, NodeState::NotVisited) {
                    self.add_step(AlgorithmStep::Edge(EdgeStep::new(edge_idx)));
                    self.dfs_helper(graph, other_node_idx);
                }
            }
        }

        self.add_step(AlgorithmStep::Node(NodeStep::new(
            node_index,
            NodeState::Visited,
        )));

        if let Some(node) = graph.node_weight_mut(node_index) {
            node.set_state(NodeState::Visited)
        }
    }