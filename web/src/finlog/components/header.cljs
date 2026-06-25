;; @file    <finlog/components/header.cljs>
;; @author  <wakaranakattari@gmail.com>
;; @info    <shared header component with navigation and username display>
;; @version <1.0>

;; @secstart->@secname <nsrq>
(ns finlog.components.header
  (:require [re-frame.core :as rf]))
;; @secend->@secname <nsrq>

;; @secstart->@secname <header>
  ;; @funcinfo <app header with title, page navigation buttons and current username>
(defn header []
  [:header.header
   ;; @secstart->@secname <headertitle> :: @secinfo <app title>
   [:div.title [:p "Finlog"]]
   ;; @secend->@secname <headertitle>

   ;; @secstart->@secname <headernav> :: @secinfo <page navigation buttons>
   [:nav.buttons-menu
    [:button {:on-click #(rf/dispatch [:set-page :dashboard])}  "Dashboard"]
    [:button {:on-click #(rf/dispatch [:set-page :expenses])}   "Expenses"]
    [:button {:on-click #(rf/dispatch [:set-page :statistics])} "Statistics"]]
   ;; @secend->@secname <headernav>

   ;; @info <displays current username from re-frame store>
   [:div.session-id [:span @(rf/subscribe [:username])]]])
;; @secend->@secname <header>