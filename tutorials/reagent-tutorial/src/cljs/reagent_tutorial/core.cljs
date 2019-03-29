(ns reagent-tutorial.core
  (:require
   [reagent.core :as reagent]))

(defn comment-list
  []
  [:div.commentList
   "Hello, world! I am a CommentList"])

(defn comment-form
  []
  [:div.commentForm
   "Hello, world! I am a CommentForm"])

(defn comment-item
  [props & children]
  [:div.comment
   (into [:p.commentAuthoer {:style (:style props)} (:author props)]
         children)])

(defn comment-box
  []
  [:div.commentBox
   [comment-item {:author "Pete Hunt"} "This is one comment"]
   [comment-item {:author "Jordan Walke" :style {:font-weight 'bold}} "This is *author* comment"]
   [comment-item {:author "author" :style {:color "#FF0000"}} "This is my comment"]])


;; -------------------------
;; Initialize app

(defn mount-root []
  (reagent/render [comment-box] (.getElementById js/document "app")))

(defn init! []
  (mount-root))
