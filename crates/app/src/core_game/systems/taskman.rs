use crate::*;

pub fn tick_taskman(gs: &mut GameState) {
	if gs.debug.taskman_empty_hands {
		tick_empty_hands(gs);
	}

	if gs.debug.taskman_give_work {
		tick_give_work(gs);
	}
}

pub fn tick_give_work(gs: &mut GameState) {
	tick_give_work_factories(gs);
	tick_give_work_harvest(gs);
}

pub fn tick_give_work_factories(gs: &mut GameState) {
	let mut crab_ids = gs //_
		.crablets
		.iter()
		.enumerate()
		.filter_map(|(i, c)| (c.navigation.status().is_idle() && c.task.is_none() && c.cargo.is_none()).then_some(i))
		.collect_vec();

	// TODO: by home area, assign only to nearby

	let destinations = gs
		.factories
		.iter()
		.enumerate()
		.sorted_by_key(|(_i, fac)| fac.priority)
		.rev()
		.flat_map(|(i, fac)| fac.inputs.iter().filter_map(move |input| (!input.is_virtually_full()).then_some((i, input.resource()))))
		.collect_vec();

	let sources = gs
		.factories
		.iter()
		.enumerate()
		.flat_map(|(i, fac)| fac.outputs.iter().filter_map(move |output| (!output.is_vitrually_empty()).then_some((output.resource(), i))))
		.collect_grouped();

	let is_pile = |factories: &[Factory], i: usize| factories[i].is_pile();

	let pile_to_pile_ok = |factories: &[Factory], src_i, dst_i| !(is_pile(factories, src_i) && is_pile(factories, dst_i));

	let find_src_for = |factories: &[Factory], resource, dst_i| {
		sources //_
			.get(&resource)?
			.iter()
			.find(|&&i| i != dst_i && factories[i].can_reserve_output(resource) && pile_to_pile_ok(factories, i, dst_i))
			.copied()
	};

	'next_destination: for &(dst_i, resource) in &destinations {
		while gs.factories[dst_i].can_reserve_input(resource) {
			let Some(src_i) = find_src_for(&gs.factories, resource, dst_i) else { continue 'next_destination };
			if gs.factories[src_i].can_reserve_output(resource) {
				let Some(crab_id) = crab_ids.pop() else { return }; // out of crabs
				println!("crab {crab_id}: {resource:?} {:?} ==> {:?}", gs.factories[src_i], gs.factories[dst_i]);
				gs.factories[src_i].reserve_output(resource).expect("bug");
				gs.factories[dst_i].reserve_input(resource).expect("bug");
				gs.crablets[crab_id].task = Task::FactoryPickUp {
					resource,
					from: gs.factories[src_i].position,
					to: gs.factories[dst_i].position,
				};
			} else {
				break;
			}
		}
	}
}

pub fn tick_give_work_harvest(gs: &mut GameState) {
	let crab_ids = gs //_
		.crablets
		.iter()
		.enumerate()
		.filter_map(|(i, c)| (c.navigation.status().is_idle() && c.task.is_none() && c.cargo.is_none()).then_some(i))
		.collect_vec();

	for i in crab_ids {
		let search_area = Bounds::from_point(gs.crablets[i].home_area).add_margin(gs.search_radius);

		let Some((near_resource, resource_pos)) = search_area
			.iter_excl()
			.filter_map(|pos| (gs.resource_reservations.at_or_default(pos) == 0).then(|| gs.resources.at_or_default(pos).zip(Some(pos))).flatten())
			.min_by_key(|(_res, pos)| pos.distance_squared(gs.crablets[i].home_area))
		else {
			gs.stats.inc(Event::GiveWorkErr);
			continue;
		};

		let Some((near_factory_id, factory_pos)) = gs
			.factories
			.iter()
			.enumerate()
			.filter(|(_i, fac)| fac.can_reserve_input(near_resource) && search_area.contains(fac.position))
			.min_by_key(|(_i, fac)| fac.position.distance_squared(gs.crablets[i].home_area))
			.map(|(i, fac)| (i, fac.position))
		else {
			gs.stats.inc(Event::GiveWorkErr);
			continue;
		};

		debug_assert!(gs.reserve_resource(resource_pos));
		debug_assert!(gs.factories[near_factory_id].reserve_input(near_resource).is_ok());
		debug_assert!(gs.crablets[i].task.is_none());

		gs.stats.inc(Event::GiveWorkOk);
		gs.crablets[i].task = Task::Harvest {
			resource: near_resource,
			from: resource_pos,
			to: factory_pos,
		};
	}
}

/// Idle crabs still carrying cargo: Task drop-off went wrong.
/// Crab must be quite near the place they were supposed to drop.
/// So try to drop nearby.
pub fn tick_empty_hands(gs: &mut GameState) {
	// All crabs that are carrying something
	let crab_ids = gs //_
		.crablets
		.iter()
		.enumerate()
		.filter(|(_i, c)| (c.navigation.status().is_idle() && c.cargo.is_some()))
		.filter(|(_i, c)| match c.task {
			Task::None => true,
			_ => false,
			//Task::Transfer { resource, .. } => Some(resource) != c.cargo, // << TODO: should drop task!
			//Task::DeliverCargo { .. } => true,
		})
		.map(|(i, _c)| i)
		.collect_vec();

	// All factories that can accept something.
	// TODO: group by resource
	let factory_ids = gs //_
		.factories
		.iter()
		.enumerate()
		.filter_map(|(i, f)| f.inputs.iter().any(|i| !i.is_virtually_full()).then_some(i))
		.collect_vec();

	for crab_id in crab_ids {
		if let Some(cargo) = gs.crablets[crab_id].cargo {
			// Go drop off at the nearest factory
			let fac_i = factory_ids
				.iter()
				.copied()
				.filter(|&fac_i| gs.factories[fac_i].can_reserve_input(cargo)) // <<< TODO: and distance is reasonable, don't go drop off across the continent
				.min_by_key(|&fac_i| gs.factories[fac_i].position.distance_squared(gs.crablets[crab_id].position));

			let Some(fac_i) = fac_i else { continue };

			if gs.factories[fac_i].reserve_input(cargo).is_ok() {
				// 👆 don't send more than we can chew
				gs.crablets[crab_id].task = Task::FactoryDeliver { to: gs.factories[fac_i].position };
			}
		}
	}
}
