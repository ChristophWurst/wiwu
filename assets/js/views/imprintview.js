/**
 * Winzerhof Wurst – www.winzerhof-wurst.at
 *
 * This file is licensed under the Affero General Public License version 3 or
 * later. See the COPYING file.
 *
 * @author Christoph Wurst <christoph@winzerhof-wurst.at>
 * @copyright Christoph Wurst 2018
 */

define(function (require) {
	'use strict';

	var Marionette = require('backbone.marionette');

	var ImprintTemplate = require('../templates/imprint.html');

	/**
	 * @class ImprintView
	 */
	var ImprintView = Marionette.ItemView.extend({
		template: ImprintTemplate,
	});

	return ImprintView;
});
