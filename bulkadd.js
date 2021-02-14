studio.menu.addMenuItem({
	name: "Bulk Add",
	execute: bulkadd,
})

function bulkadd() {
	var track = studio.window.editorCurrent();
	var files = studio.window.browserSelection();
	if (track == null || files.length < 1) {
		error("You must select one or more assets and a track.");
		return;
	}
	
	if (!track.isOfType('GroupTrack')) {
		error("You must select a track in the editor pane.");
		return;
	}
	
	for (var i = 0; i < files.length; i++) {
		if (!files[i].isOfType('AudioFile')) {
			error("You must select only audio files in the browser");
			return;
		}
	}
	
	// This API is Very Bad and does not provide a way to get parameter names
	// The preset ID is the same for all sixteenth_note parameters so we just search for that
	var presetId = '{c2b359ee-617f-49f2-a8bd-c627a9f4902b}';
	var possibleParams = studio.window.editorCurrent().event.parameters;
	var parameter;
	
	for (var i = 0; i < possibleParams.length; i++) {
		if (possibleParams[i].preset.id == presetId) {
			parameter = possibleParams[i];
			break;
		}
	}
	
	if (parameter == undefined) {
		error("Could not find parameter with preset ID " + presetId);
		return;
	}
	
	for (var i = 0; i < files.length; i++) {
		var sound = track.addSound(parameter, 'SingleSound', i + 1, 1);
		sound.audioFile = files[i];
	}
}

function error(reason) {
	studio.ui.showModalDialog({
        windowTitle: "Error",
        widgetType: studio.ui.widgetType.Layout,
		layout: studio.ui.layoutType.VBoxLayout,
		items: [
			{ widgetType: studio.ui.widgetType.Label, text: reason },
			{
                widgetType: studio.ui.widgetType.Layout,
                layout: studio.ui.layoutType.HBoxLayout,
                contentsMargins: { left: 0, top: 12, right: 0, bottom: 0 },
                items: [
                    { widgetType: studio.ui.widgetType.Spacer, sizePolicy: { horizontalPolicy: studio.ui.sizePolicy.MinimumExpanding } },
                    { widgetType: studio.ui.widgetType.PushButton, text: "OK", onClicked: function() { this.closeDialog(); } },
                ],
            },
		]
	})
}