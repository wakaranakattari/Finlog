;; @file    <finlog/core.cljs>
;; @author  <wakaranakattari@gmail.com>
;; @info    <app entry point with re-frame store, routing and root component>
;; @version <1.0>

;; @secstart->@secname <nsrq>
(ns finlog.core
  (:require [reagent.dom :as rdom]
            [re-frame.core :as rf]
            [cljs.core.async :refer [go <!]]
            [finlog.api :as api]
            [finlog.components.header :as header]
            [finlog.components.dashboard :as dashboard]
            [finlog.components.expenses :as expenses]
            [finlog.components.statistics :as statistics]))
;; @secend->@secname <nsrq>

;; @secstart->@secname <events>
  ;; @info <initialize app db with default state>
(rf/reg-event-db
 :init
 (fn [_ _] {:expenses [] :username "" :page :dashboard :loading? true}))

  ;; @info <replace expenses vector in db>
(rf/reg-event-db
 :set-expenses
 (fn [db [_ v]] (assoc db :expenses v)))

  ;; @info <replace username string in db>
(rf/reg-event-db
 :set-username
 (fn [db [_ v]] (assoc db :username v)))

  ;; @info <switch active page and trigger 1s loading state>
(rf/reg-event-db
 :set-page
 (fn [db [_ v]]
   (rf/dispatch [:set-loading true])
   (js/setTimeout #(rf/dispatch [:set-loading false]) 1000)
   (assoc db :page v)))

  ;; @info <set loading flag in db>
(rf/reg-event-db
 :set-loading
 (fn [db [_ v]] (assoc db :loading? v)))
;; @secend->@secname <events>

;; @secstart->@secname <subscriptions>
  ;; @info <subscriptions for expenses, username, current page and loading state>
(rf/reg-sub :expenses (fn [db _] (:expenses db)))
(rf/reg-sub :username (fn [db _] (:username db)))
(rf/reg-sub :page     (fn [db _] (:page db)))
(rf/reg-sub :loading? (fn [db _] (:loading? db)))
;; @secend->@secname <subscriptions>

;; @secstart->@secname <loaddata>
  ;; @funcinfo <async loads expenses and username from api, dispatches results to store>
(defn load-data! []
  (go
    (rf/dispatch [:set-loading true])
    (let [expenses (<! (api/load-expenses))
          username (<! (api/load-username))]
      (rf/dispatch [:set-expenses expenses])
      (rf/dispatch [:set-username username])
      ;; @info <delay hiding loader to avoid flash of unstyled content>
      (js/setTimeout #(rf/dispatch [:set-loading false]) 1000))))
;; @secend->@secname <loaddata>

;; @secstart->@secname <currentpage>
  ;; @funcinfo <returns the component for the currently active page>
(defn current-page []
  (case @(rf/subscribe [:page])
    :dashboard  [dashboard/dashboard]
    :expenses   [expenses/expenses-page]
    :statistics [statistics/statistics-page]))
;; @secend->@secname <currentpage>

;; @secstart->@secname <preloader>
  ;; @funcinfo <full screen loading spinner shown during data fetch and page transitions>
(defn preloader []
  [:div.preloader
   [:div.preloader-spinner]
   [:div.preloader-text "Finlog"]])
;; @secend->@secname <preloader>

;; @secstart->@secname <app>
  ;; @funcinfo <root component rendering header and either preloader or current page>
(defn app []
  (let [loading? @(rf/subscribe [:loading?])]
    [:div
     [header/header]
     ;; @info <show preloader during transitions, otherwise render active page>
     (if loading?
       [preloader]
       [current-page])]))
;; @secend->@secname <app>

;; @secstart->@secname <init>
  ;; @funcinfo <initializes re-frame store, loads data and mounts root component to dom>
(defn init []
  (rf/dispatch-sync [:init])
  (load-data!)
  (rdom/render [app] (.getElementById js/document "app")))
;; @secend->@secname <init>