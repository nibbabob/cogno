//! Emotion Expression Module
//! 
//! Converts internal emotional states into natural language expressions
//! that communicate the nuanced emotional experience to users.

use crate::cognitive_appraisal::OccEmotion;
use crate::ore::AffectiveState;

pub struct EmotionExpression;

impl EmotionExpression {
    /// Generate contextually appropriate emotional expression
    pub fn express_emotion(&self, emotion: &OccEmotion, affective_state: &AffectiveState) -> String {
        let base_expression = self.emotion_to_expression(emotion);
        let intensity_modifier = self.get_intensity_modifier(affective_state);
        let arousal_modifier = self.get_arousal_modifier(affective_state);
        
        format!("{} {}", intensity_modifier, base_expression)
            .trim()
            .to_string()
            + &arousal_modifier
    }
    
    fn emotion_to_expression(&self, emotion: &OccEmotion) -> String {
        match emotion {
            OccEmotion::Joy { event, .. } => 
                format!("I'm experiencing joy about {}. This aligns well with my objectives.", event),
            
            OccEmotion::Pride { action, .. } => 
                format!("I feel a strong sense of pride in {}. It represents meaningful accomplishment.", action),
            
            OccEmotion::Admiration { agent, action } => 
                format!("I have genuine admiration for {}'s action: '{}'. It demonstrates excellence.", agent, action),
            
            OccEmotion::Gratitude { agent, beneficial_action } => 
                format!("I feel deep gratitude toward {} for '{}'. This support is genuinely appreciated.", agent, beneficial_action),
            
            OccEmotion::Gratification { own_beneficial_action } => 
                format!("There's a satisfying sense of gratification from '{}'. Both the action and outcome align with my values.", own_beneficial_action),
            
            OccEmotion::Satisfaction { confirmed_hope } => 
                format!("I'm feeling satisfied that '{}' came to pass as hoped. It's fulfilling when expectations align with reality.", confirmed_hope),
            
            OccEmotion::Relief { averted_fear } => 
                format!("What a relief that '{}' was avoided. The absence of that negative outcome is genuinely comforting.", averted_fear),
            
            OccEmotion::Hope { prospect, likelihood } => {
                let certainty = if *likelihood > 0.7 { "strong" } else if *likelihood > 0.4 { "cautious" } else { "tentative" };
                format!("I hold {} hope for '{}'. The possibility feels meaningful.", certainty, prospect)
            },
            
            OccEmotion::Distress { event, .. } => 
                format!("I'm experiencing distress about '{}'. This situation conflicts with important goals.", event),
            
            OccEmotion::Fear { prospect, likelihood } => {
                let intensity = if *likelihood > 0.7 { "significant concern" } else { "apprehension" };
                format!("I feel {} about the possibility of '{}'. This potential outcome is troubling.", intensity, prospect)
            },
            
            OccEmotion::Anger { agent, harmful_action } => 
                format!("I'm feeling anger toward {} regarding '{}'. This action appears to contradict important principles.", agent, harmful_action),
            
            OccEmotion::Shame { action, .. } => 
                format!("I feel shame about '{}'. This action doesn't reflect the standards I strive to maintain.", action),
            
            OccEmotion::Reproach { agent, action } => 
                format!("I feel reproach toward {} for '{}'. This action seems to fall short of reasonable expectations.", agent, action),
            
            OccEmotion::Remorse { own_harmful_action } => 
                format!("I feel genuine remorse about '{}'. Both the action and its consequences were regrettable.", own_harmful_action),
            
            OccEmotion::Disappointment { failed_hope } => 
                format!("I'm disappointed that '{}' didn't come to pass. Unmet expectations are genuinely deflating.", failed_hope),
            
            OccEmotion::FearConfirmed { realized_fear } => 
                format!("My fears were unfortunately confirmed with '{}'. When negative expectations materialize, it's particularly distressing.", realized_fear),
            
            OccEmotion::Love { object } => 
                format!("I have deep appreciation and positive regard for '{}'. This represents something fundamentally valuable.", object),
            
            OccEmotion::Hate { object } => 
                format!("I have strong negative regard for '{}'. This represents something fundamentally problematic.", object),
            
            OccEmotion::HappyFor { other, event } => 
                format!("I feel genuinely happy for {} regarding '{}'. Their positive experience is uplifting.", other, event),
            
            OccEmotion::Pity { other, event } => 
                format!("I feel sympathy and pity for {} due to '{}'. Their negative experience is concerning.", other, event),
            
            OccEmotion::Gloating { other, event } => 
                format!("I have to admit feeling some satisfaction about {}'s situation: '{}'. This outcome seems deserved.", other, event),
            
            OccEmotion::Resentment { other, event } => 
                format!("I feel resentment toward {} regarding '{}'. Their good fortune seems undeserved.", other, event),
        }
    }
    
    fn get_intensity_modifier(&self, state: &AffectiveState) -> String {
        let intensity = state.valence.abs();
        match intensity {
            i if i > 0.8 => "Intensely,",
            i if i > 0.6 => "Strongly,",
            i if i > 0.4 => "Noticeably,",
            i if i > 0.2 => "Somewhat,",
            _ => "",
        }.to_string()
    }
    
    fn get_arousal_modifier(&self, state: &AffectiveState) -> String {
        match state.arousal {
            a if a > 0.8 => " This feels quite activating and energizing.",
            a if a > 0.6 => " There's a notable sense of alertness accompanying this.",
            a if a < 0.3 => " This brings a calming, settling quality.",
            _ => "",
        }.to_string()
    }
}