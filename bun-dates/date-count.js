/// date-count - simple Scriptable (iOS app) widget to count days since last incident
// TODO: Store incidentDates in a textfile

const appContext = {
	// Add new incidents in ISO8601 format. Order does not matter
	incidentDates: [
		// Initial
		"2025-10-07",
	],
	// How frequently to refresh widget in hours
	refreshHoursInterval: 6,
};

const differenceInDays = (dateA, dateB) => {
	const diffInMs = dateA - dateB;
	return Math.floor(diffInMs / (1000 * 60 * 60 * 24));
};

const daysSinceIncident = (dates, date) => {
	const mostRecent = dates.sort((a, b) => new Date(b) - new Date(a))[0];
	const mostRecentDate = new Date(mostRecent);
	return differenceInDays(date, mostRecentDate);
};

const main = (ctx) => {
	const today = new Date();
	const daysDelta = daysSinceIncident(ctx.incidentDates, today);

	let w = new ListWidget();
	w.refreshAfterDate = new Date(
		Date.now() + 1000 * 60 * 60 * ctx.refreshHoursInterval,
	);

	const printText = (() => {
		if (ctx.incidentDates.length === 0) return "no incidents recorded";
		if (daysDelta < 0) return "0 days since";
		return `${daysDelta} days since`;
	})();

	let text = w.addText(printText);
	text.font = Font.mediumMonospacedSystemFont(18);
	text.textColor = Color.white();
	w.backgroundColor = new Color("#1E1E1E");
	text.centreAlignText();

	// If running outside of the widget env, show widget in a medium size
	if (!config.runsInWidget) {
		w.presentMedium();
	}

	Script.setWidget(w);
	Script.complete();
};

main(appContext);
