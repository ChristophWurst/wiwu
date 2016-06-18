/**
 * Winzerhof Wurst – www.winzerhof-wurst.at
 *
 * This file is licensed under the Affero General Public License version 3 or
 * later. See the COPYING file.
 *
 * @author Christoph Wurst <christoph@winzerhof-wurst.at>
 * @copyright Christoph Wurst 2016
 */

define(function (require) {
    'use strict';

    var Backbone = require('backbone');

    var Wine = require('models/wine');

    /**
     * @class WineController
     */
    var WineCollection = Backbone.Collection.extend({
        model: Wine,
        url: undefined,
        initialize: function (options) {
            this.url = options.url;
        }
    });

    return WineCollection;
});