//  ogmarkup -- a markup language for story writers
//  Copyright (C) <year>  <name of author>
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.

#[macro_use]
extern crate criterion;
extern crate ogmarkup;

use criterion::black_box;
use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse", |b| b.iter(|| ogmarkup::parse(black_box(INPUT))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

const INPUT: &'static str = r#"_______________personnage__________________
                               Sanaa
__________________________________________

[Oh, Digne,| souffla Sanaa et elle ressentait tant et tant de choses subitement.|](Sanaa) Elle sentit ses yeux s’embuer légèrement et préféra détourner le regard le temps de reprendre un semblant de contenance.

<Il va croire qu’il m’a contrariée|, songea-t-elle confusément et son cœur se serra un peu plus rien qu’en y pensant.|>(Sanaa) S’il était une chose que l’adolescente ne voulait surtout pas, c’était que le jeune Hadjaoui culpabilisât par sa faute. <Il est celui qui devrait être contrarié, pas moi. *Lui*.>(Sanaa)

Elle n’était pas venue pour *ça*. Elle avait décidé de s’éclipser de la demeure des Noblegriffon pour rejoindre un soupirant un peu perdu et pour comploter avec lui sur la meilleure façon de séduire la jouvencelle qui occupait toutes ses pensées ; c’était un tout autre scénario qui était en train de se dérouler bien malgré elle. Il avait suffi de quelques mots à l’ancien esclave pour ramener Sanaa à tout ce qu’elle ne lui avait pas dit. Ces mensonges par omission qu’elle avait laissés s’accumuler entre eux étaient comme un fossé qui l’empêchait de simplement recevoir la touchante confession de son ami comme elle l’aurait dû.

<Jure-moi|, l’avait supplié Aislinn avant d’accepter que la Vaanie l’accompagnât dans sa quête pour dénicher Aimé Hadjaoui.| Jure-moi que, si nous le retrouvons, jamais tu ne lui confieras la réalité de ta condition.>(Aislinn) Aislinn ne prononçait jamais le mot «esclave»; ce n’était pas de Sanaa dont l’héritière Noblegriffon, mais bien d’elle-même et de son impuissance à l’affranchir ainsi qu’elle le lui avait promis. La Vaanie, qui n’aimait rien moins que de s’avouer asservie, avait consenti à sa requête sans la moindre hésitation. Comme elle le regrettait, tandis qu’elle fuyait le regard de Digne !

<Il ne me connaît pas|, réalisa-t-elle avec horreur.| Il s’est convaincu du contraire, mais a comblé chacun de mes silences de tant de fausses vérités ; c’est comme si j’avais passé mon temps à lui mentir.>(Sanaa)

Cela devait cesser.

[Je ne suis pas celle que tu crois,| annonça-t-elle de but en blanc en redressant la tête.|](Sanaa) Elle dardait désormais sur lui des prunelles résolues, bien qu’encore par trop humides. [Je ne possède rien... Les robes que je porte, les bijoux avec lesquels je parade, les montures que je chevauche, l’or que je dépense… Tout est à Aislinn,| expliqua-t-elle d’une traite.|](Sanaa)

<J’appartiens à Aislinn|, hurla-t-elle silencieusement, mais aucun son de franchit cette fois les défenses de ses lèvres scellées.|>(Sanaa) Elle comprit qu’elle garderait ce secret-là par-devers elle encore un petit peu.

[Elle ne m’a jamais rien demandé — c’est même tout le contraire, elle m’a tant et tant donné —, mais officiellement, j’ai été placée à son service par son tuteur.  Je tenais à ce que tu le saches,| continua-t-elle plus doucement.| Parce que…](Sanaa) Elle haussa les épaules et sécha ses larmes d’une main un peu tremblante. [Parce que tu es mon ami.](Sanaa) Elle marqua une rapide pause, embrassa la pièce de vie des Hadjaoui du regard, puis lâcha un petit rire qui lui permit d’étouffer un sanglot chargé d’émotion. [Puisque l’on en est aux confessions, je trouve votre foyer bien plus chaleureux que la demeure de Varlar… et si je connais Aislinn aussi bien que je *sais* la connaître, je suis prête à parier qu’elle sera de mon avis le jour où tu l’inviteras à venir ici.](Sanaa)
____________________personnage___________________
                                          Sanaa
________________________________________________

[Tu es idiot,| répondit-elle avec un nouveau hoquet où le rire prenait cette fois le pas sur les pleurs.| Je sais me tenir.](Sanaa)

Et dans le même temps, en d’autres circonstances, elle ne pouvait pas affirmer qu’elle aurait effectivement joué le jeu *jusqu’au bout*. Ses sentiments pour Digne était clairs, le jeune homme n’en demeurait pas moins tout à fait attirant. Bien malgré elle, son imagination l’emmena dans l’ambiance feutrée et tamisée d’une chambre à coucher, où elle se vit nue contre son torse. Cette simple pensée la troubla plus qu’elle ne voudrait bien jamais le dire et elle sentit ses joues picoter. Heureuse que son ami ne pût pas déceler son émoi de par leur étreinte, elle enfouit son visage dans son cou et commença à pouffer nerveusement. Son souffle chaud sur sa peau dut le chatouiller, car il se mit à gigoter en protestant.

Il n’en fallut guère plus pour que les deux Vaanis partissent sur un fou rire qui les força à se séparer. Cela suffit pour chasser les dernières voluptés érotiques qui embrumaient encore l’esprit par ailleurs agité de Sanaa.

[Oh, par les Dieux,| soupira-t-elle en tentant de reprendre sa respiration.| Si j’avais su que notre petite conversation serait aussi épuisante…](Sanaa) Elle lui lança son plus beau sourire, mettant ainsi fin à l’étrange interlude qu’avait provoqué l’adorable confession du Thaari. [Merci, en tout cas,| lui souffla-t-elle avec gratitude.|](Sanaa) Ses yeux pétillèrent avec malice quand elle reprit, à nouveau égale à elle-même : [Avec tout cela, il ne faudrait pas oublier pourquoi je me suis invitée chez toi. Aislinn ! Il est grand temps que tu arrêtes de la *regarder* et que tu t’attelles à la *séduire*.](Sanaa) Et l’adolescente d’ajouter, adorablement mutine : [Ça tombe bien, j’ai une grande expérience en la matière…](Sanaa)
_______________personnage_______________
                      Aislinn Noblegriffon
_______________________________________

__________________date__________________
Du 40 Barkios au Kÿrianos 19 Verimios de l’An `13:XI`, dans la populeuse cité de Thaar.
_______________________________________

De l’escapade de Sanaa le soir qui suivit son premier entraînement d’escrime avec Digne, Aislinn ne sut jamais grand-chose ; elle devina sans trop de peine que la Vaanie avait dû rejoindre le benjamin des Hadjaoui, mais de ce qu’ils firent ou se dirent, cela demeura un mystère. La Rivoise ne tarda guère à s’imaginer que les deux amis fussent en fait plus que cela et, loin de s’en formaliser, se réjouit de leur bonheur supposé. Pour ne pas embarrasser Sanaa, elle prit bien soin de ne jamais aborder avec elle leur secret de polichinelle. <Il viendra bien un temps où elle se décidera à m’en parler d’elle-même|, estimait-elle peut-être un peu naïvement.|>(Aislinn)

Dans les ennéades qui suivirent, Aislinn poursuivit son initiation en escrime, guidée par un Digne qui prenait à chaque fois plus d’assurance. L’héritière Noblegriffon le découvrait pendant leurs séances sous un jour nouveau, bien éloigné du jeune homme discret et au regard fuyant qu’elle connaissait. Force était de constater qu’elle appréciait cette nouvelle facette de sa personnalité ; elle n’avait jamais vraiment compris l’étrange réserve dont il ne se départissait jamais lorsqu’il lui adressait la parole et était heureuse qu’il adoptât une attitude plus détendue. Elle ignorait si elle devait ce changement à sa romance naissante — supposée — avec Sanaa ou au fait qu’Aimé était débordé et n’avait plus toujours le loisir d’assister à leurs joutes amicales. Elle avait remarqué que Digne était souvent un peu plus serein quand il n’était pas là pour le voir jouer les professeurs. Peu importait, au fond.

Aislinn se convainquit rapidement qu’elle ne serait jamais une bretteuse exceptionnelle — elle pouvait désormais estimer le temps et les efforts que cela lui demanderait et savait pertinemment qu’elle manquerait du premier et préférait réserver les seconds à d’autres ambitions —, mais acquérir les bases d’un art qui lui permettrait peut-être un jour de ne pas se sentir sans défense face à un danger imminent avait quelque chose de grisant. Elle se rendit vite compte, par exemple, qu’elle supportait mieux les rares colères de Varlar depuis qu’elle était parvenue à passer sous la garde de Digne pour la première fois.

Quand ils en avaient terminé avec les épées de bois, ils allaient s’asseoir contre le mur de l’écurie de l’auberge dans laquelle travaillaient les Hadjaoui et ils se racontaient ce qu’ils avaient fait depuis leur dernière rencontre. Digne et Aislinn commencèrent à se confier de plus en plus de choses.  Aislinn lui avoua ses doutes naissantes à l’encontre des pratiques commerciales de Varlar, mais lui parla aussi de son enfance à Rive, de sa mère et décrivit quelques-unes de leurs «aventures». Les rumeurs des pérégrinations de la gardienne de Tyra arrivaient au compte-goûte à Thaar. La mort du Roi de Naelis de sa main avait fait grand bruit l’année passée, mais depuis, il semblait que Katalina Noblegriffon avait disparu. L’ancien esclave, de son côté, s’ouvrit un peu sur ses années de servitude ; Aislinn fit son possible pour l’écouter avec respect.

Bien sûr, pendant tout ce temps, Aislinn passa aussi beaucoup de temps avec l’aîné de la fratrie Hadjaoui. Convaincue que Sanaa et Digne filaient un amour épanoui, elle commença à questionner ses propres sentiments envers Aimé. Il la troublait, assurément. Quand il lui faisait un compliment, elle rougissait et son cœur battait un peu plus fort. Elle gardait un souvenir impérissable — et sans doute légèrement fantasmé — de leur rencontre dans l’Agora d’Ys. Les rares fois où il l’avait prise dans ses bras, elle s’y était sentie si bien qu’elle avait fermé les yeux et espéré que le temps s’arrêta ; et en même temps, elle n’avait pas voulu une seule seconde que leurs câlins allassent plus loin et l’idée, par exemple, qu’il l’embrassât la mettait extrêmement mal à l’aise. Quant à s’imaginer nue contre lui…

[Je crois que je l’adore comme un frère,| finit-elle par avouer à Sanaa un jour sans crier gare,| mais comment m’en assurer ?](Aislinn)

La Vaanie n’avait guère trouvé à répondre à ses atermoiements et Aislinn n’avait pas insisté. Elle oubliait trop aisément que Sanaa était d’un an sa cadette et n’y connaissait guère plus qu’elle en amours. Surtout, elle devait avoir l’esprit bien occupé par ses propres histoires de cœur, dont elle ne lui disait toujours rien. Cela commençait à questionner et à inquiéter la Rivoise, en réalité ; elle en venait à se demander si elle n’avait pas inventé toute cette histoire. D’autant que l’attitude de Digne était de plus en plus ambiguë et qu’elle avait eu plus d’une fois l’impression qu’il essayait de la séduire, elle ! Elle ne savait plus ce qui la troublait le plus : que Sanaa lui tût la nature de ses fréquentes entrevues avec le Hadjaoui, que Digne se montra ouvertement charmeur… ou qu’elle se découvrit sensible à ses avances supposées.

Quand Verimios succéda à Barkios, un événement vint boulverser la paisible routine de la Noblegriffon et accapara son esprit tant et si bien qu’elle bouda ses visites régulières aux Hadjaoui. C’était, ainsi que l’expliqua Sanaa en quelques mots aux deux frères, que l’homme qu’elle avait envoyé en Péninsule plusieurs ennéades plus tôt en était enfin revenu avec le précieux trésor qu’il avait été chargé de ramener. Depuis, Aislinn n’avait pratiquement pas quitté sa chambre, depuis laquelle elle dévorait la copie des mémoires que Katalina Noblegriffon avait dictées à une nonne de Néera peu de temps avant de trouver refuge à Loqriv.

[Je ne sais pas ce qu’elle espère y trouver,| soupira Sanaa et son regard trahissait toute l’inquiétude qu’elle nourrissait à voir son amie se couper du reste du monde.| Je ne crois pas qu’elle le sache elle-même.](Sanaa)

Cet étrange interlude s’étala sur deux ennéades entières, durant lesquelles Aislinn relut quatre fois l’intrigant manuscrit. La nonne avait tâché de retranscrire le plus fidèlement possible les propos de Katalina Noblegriffon, qui à l’époque déjà était réputée pour rarement faire sens. Bien sûr, sa fille adoptive avait vécu à ses côtés pendant plusieurs années et elle s’était vite rendu compte que cela l’avantageait sur les quelques érudits qui l’avaient précédée.

Le premier jour de la troisième ennéade de Verimios, elle émergea finalement de sa retraite, le cœur lourd de tout ce qu’elle avait découvert et avide de s’en ouvrir à quelqu’un. Comme elle regrettait l’absence de Pierre et le peu d’efforts qu’elle avait consenti à faire pour rester en contact avec lui ! *Lui* aurait compris ce qu’elle était en train de vivre. Le premier réflexe de Sanaa fut de chercher Sanaa, mais elle se souvint vite que la Vaanie nourrissait des sentiments conflictuels pour la gardienne. Aislinn pensait qu’elle en avait peur. Elle exclut sans remords Varlar, dont elle commençait à se méfier de plus en plus. Elle songea bien à aller trouver Aimé, car le Hadjaoui avait rencontré la Serramiroise et avait touché du doigt tout le mystère qui l’enveloppait. Avant qu’elle n’arrêtât complètement sa décision pourtant, un autre visage s’imposa à elle et la fit hésiter.

Sans qu’elle s’en fût vraiment rendu compte, Digne et elle s’étaient beaucoup rapprochés. Le sourire plein de douceur et les encouragements toujours justes du jeune homme lui manquaient terriblement. Pis ! Comme elle n’avait pas daigné le prévenir de ses absences, elle prit peur à l’idée de les avoir perdu. <Sanaa lui aura expliqué|, se répéta-t-elle nerveusement, seulement pour sentir sa culpabilité s’accentuer encore un peu plus.|>(Sanaa) Ils étaient tous deux ses plus proches et tendres amis et elle les avait bien mal traités en retour.

Le hasard voulait que ce jour-là fût — ou aurait dû être, si elle n’avait pas mis fin à leur petite routine — de ceux où ils s’entraînaient habituellement. Sur un coup de tête et bien qu’elle fût déjà en retard, elle décida d’aller à sa rencontre. Quelques passes d’armes l’aideraient assurément à faire le vide dans son esprit… et elle pourrait surtout s’excuser. Et se confier, s’il la laissait faire.

Quand elle se présenta devant l’auberge où travaillaient les Hadjaoui, son cœur se serra. Digne l’attendait. Son regard s’alluma d’émotions qu’elle préféra ne pas affronter en détournant ses prunelles honteuses.

[J’espère que je n’arrive pas trop tard pour un cours particulier,| énonça-t-elle en guise de salut.|](Aislinn) Et la Noblegriffon d’ajouter, après une pause qui lui donna l’impression de s’étirer sur une éternité. [Je suis désolée de ne pas être venue plus tôt…](Aislinn)
_______________personnage_______________
                      Aislinn Noblegriffon
_______________________________________

Si les premiers mots prononcés par Digne rassénérèrent la Noblegriffon, qui se permit de darder sur lui un regard où le soulagement se mêlait à la reconnaissance, la question qu’il posa juste après la plongea dans de nouvelles affres de perplexité. Pour sa défense, l’adolescente sortait de deux ennéades éprouvantes durant lesquelles elle s’était abîmée les yeux à lire à la lueur de chandelles tremblantes les élucubrations par trop souvent abscons d’une gardienne à la raison déclinante. Elle avait l’impression de se réveiller d’un mauvais rêve et ne conservait qu’un souvenir relativement flou des jours qui avaient précédé sa réception de sa copie des mémoires de sa mère adoptive.

[Pourquoi sinon ?| l’interrogea-t-elle avec innocence.|](Aislinn) Sa confusion ne dura qu’un temps, car ses esprits furent très vite accaparés une nouvelle fois par le poids des mots contenus dans ces pages qu’elle avait parcouru avec une attention presque maladive. [C’était comme si elle me parlait, Digne,| expliqua-t-elle avec émotion.|](Aislinn) Elle aurait souhaité qu’il attendît la fin de leur séance d’entraînement pour aborder le sujet, mais maintenant qu’il l’avait fait, ses digues à elle rompaient l’une après l’autre. [Chaque phrase que j’ai lu faisait écho à un souvenir d’elle. Elle m’a dit tant et tant de choses que je n’avais pas compris…](Aislinn)

Elle n’avait pas terminé sa diatribe encore que des larmes en abondance roulaient déjà sur ses joues. Elle fit un pas hésitant en direction du jeune homme, qui semblait dépassé par les événements, avant de se précipiter contre son torse.

[Elle a souffert, Digne. Oh, par les Dieux, elle a tellement souffert. C’était là, sous mes yeux, je le voyais, je le savais, mais je ne *comprenais* pas.](Aislinn)
_______________personnage________________
                       Aislinn Noblegriffon
________________________________________

[Une enfant, oui,| sanglota-t-elle misérablement.| Une enfant stupide et aveugle.](Aislinn)

Bien sûr, la vérité était différente et, bien qu’elle ignorât de fait la nature des tourments de celle qui l’adopterait, Aislinn avait fait beaucoup pour essayer d’apaiser ses souffrances. La Rivoise avait toujours eu tendance à minimiser ses efforts et Digne ne pouvait donc pas prendre toute la mesure du fossé qui séparait ses dires de ce qui s’était vraiment passé.

Pendant plusieurs longues minutes, ses seuls mouvements furent des tremblements et les seuls sons qu’elle émit furent des sanglots étouffés. Elle n’avait pas versé la moindre larme tout le temps qu’avait duré sa retraite et découvrait, dans les bras de Digne, combien elle en avait besoin pour évacuer toute la tension qu’elle avait accumulée. Quand elle commença à s’apaiser, elle le sentit s’écarter légèrement et elle n’essaya pas à comprendre ses motivations ; elle enroula ses bras autour de lui et se lova un peu plus encore dans leur étreinte. Il n’y avait aucune ambiguïté dans ses gestes ; elle ne cherchait que de la chaleur humaine et du soutien, dont elle s’était trop longtemps privée.

Après ce qui dût sembler être une éternité à Digne, elle finit par le libérer avec un soupir qui trahissait toutes les émotions qui l’agitaient. Elle se sentait mieux, pourtant. Vidée, épuisée, mais un peu apaisée.

[Merci,| souffla-t-elle en essuyant ses larmes.|](Aislinn)

Elle n’était même pas gênée par leur promiscuité et attendit encore quelques secondes avant de s’extirper gentiment de ses bras ; le jeune thaari profita de ce laps de temps pour la couver d’un regard qui la frappa par sa tendresse. Elle ne croyait pas que quelqu’un n’eût jamais dardé sur elle des yeux pareils et cela la ramena d’un coup aux questions qu’elle pouvait se poser *avant* son ermitage. L’héritière Noblegriffon aurait voulu qu’il continuât, mais quand il caressa sa joue du bout des doigts, le visage souriant de Sanaa s’imposa à elle.

<Par les Cinq, qu’est-ce que je suis en train de faire ?| s’émut-elle.|>(Aislinn)

Elle fut soulagée qu’il n’essayât pas de la retenir et elle tâcha de retrouver le fil de ses pensées tandis qu’elle remettait un peu d’ordre dans sa tenue et sa coiffure. L’espace d’une seconde, elle avait éprouvé quelque chose d’inédit, comme une connexion qui s’établissait entre elle et Digne et cela lui avait semblé si naturel et si agréable ; mais elle ne devait pas oublier combien Sanaa appréciait, elle aussi, à l’Hadjaoui.

Digne vola à son secours en lui demandant si elle désirait se promener un peu dans les rues autour de l’auberge ; ravie de cette échappatoire, l’adolescente accepta de bon cœur, puis baissa les yeux sur ses vêtements. Elle n’avait jamais porté de pantalons avant de commencer ses leçons d’escrime et ne s’y sentait toujours pas totalement à l’aise. Sans trop savoir si elle fabulait ou non, elle avait l’impression que tous les regards convergeaient sur elle pour la juger. Elle décida pourtant de passer outre sa gêne et se laissa guider, heureuse de simplement emboîter le pas au jeune homme.

Une chose était claire : il devenait urgent qu’elle confrontât Sanaa pour enfin en apprendre un peu plus sur la nature des sentiments que la Vaanie entretenait pour Digne. Aislinn, de son côté, n’était plus certaine de rien.
______________personnage______________________________
                  Aislinn Noblegriffon
_____________________________________________________

_________________date_________________________________
Le matin de l’Elenwënas 41 Verimios de l’An `13:XI`, dans la demeure de Varlar.
_____________________________________________________

[Regarde, ici, je suis certaine que c’est de lui dont elle parle,| expliqua Aislinn avec conviction en tapotant avec insistance le passage qui l’intéressait de la pulpe de son index.|](Aislinn)

Après avoir ostensiblement exprimé sa lassitude en appuyant son soupir, Sanaa s’arracha au dossier confortable de son fauteuil pour porter des yeux déjà fatigués sur les mémoires de Katalina Noblegriffon.

[Tu te rends compte que je parle très mal le diantrais ?| lui demanda-t-elle après avoir recommencé sa lecture une troisième fois.|](Sanaa) Elle fronça les sourcils. [Attends, ces mots-là ne sont pas dans le bon ordre… Si ?](Sanaa)

[La nonne qui a retranscrit les confessions de mère a cherché à rester le plus fidèle possible à ce qu’elle lui dictait et…](Aislinn) Elle haussa les épaules avec fatalisme. [Je ne sais pas, je n’ai jamais eu tant de mal que cela à la comprendre.](Aislinn)

[Oui et bien, personnellement, je n’ai pas eu l’honneur de grandir à ses côtes,| fit valoir la Vaanie en repoussant légèrement le manuscrit vers Aislinn.| Du coup, tu veux bien me traduire ce paragraphe si important ?](Sanaa)

Et l’héritière de lever les yeux au ciel, avant de poser ses deux paumes de chaque côté de l’ouvrage, de se pencher en avant et de lire la première phrase en silence. Il lui semblait qu’elle aurait pu réciter le manuscrit les yeux fermés, mais dans le même temps, elle ne souhaitait surtout pas trahir les confessions de la gardienne avec des approximations malheureuses.

[*La première fois que je l’ai vu, il était tel un serpent, mais un qui ne savait pas que tombés étaient ses crocs.*](Aislinn) Elle coula un regard vers Sanaa. [D’accord, c’est vrai que cette formulation n’est pas forcément très… courante.](Aislinn) La Vaanie esquissa un léger sourire, avant de l’encourager d’un hochement du chef à continuer. [*Je l’ai observé rôder autour de moi ; il m’apparaissait clairement que de l’ombre il pensait avoir fait son manteau, quand en vérité ses mimiques et grimaces trahissaient tout de ses intentions.*](Aislinn)

[C’est vrai qu’il ne peut pas s’empêcher d’en faire des tonnes, quand il manigance quelque chose,| concéda Sanaa.|](Sanaa)

[Et ça continue comme ça sur plusieurs paragraphes,| poursuivit Aislinn en parcourant rapidement cette dernière pour trouver les fragments qui l’intéressaient.| *Il a cru pouvoir faire sien ce qui en rien ne lui revenait de droit, sans se rendre compte qu’il ne pourrait jamais faire fructifier ce qu’il entendait me voler ; il le dilapiderait ainsi qu’il l’avait fait tout ce qu’il avait toujours touché.* Et cette phrase, surtout : *il restera toute sa vie un piètre marchand, mais un savant escroc.*](Aislinn)

[Ce n’est rien qu’au moins l’une de nous deux avait déjà deviné,| remarqua Sanaa en retrouvant sa posture précédente dans le fauteuil.|](Sanaa)

Aislinn demeura quelques secondes immobile, perdue dans ses réflexions, avant de pousser un soupir agacé et de refermer le lourd ouvrage. C’était une sensation si désagréable que d’être convaincue que les réponses aux questions qu’elle pouvait se poser se trouvait dans ces mémoires, mais qu’un vie entière lui serait sans doute nécessaire pour parvenir à toutes les décrypter. Sanaa comprit de toute évidence vers quelle pente son amie commençait à glisser et décida qu’elles n’avaient passé que trop de temps à s’abîmer les yeux.

[Je sais ce que tu es en train de faire,| annonça-t-elle avec juste ce qu’il fallait d’amusement dans la voix.|](Sanaa)

[Ah ?| l’interrogea la Rivoise en s’arrachant à la contemplation de la couverture du grimoire pour mieux darder des prunelles surprises sur la Vaanie.|](Aislinn) <Je ne le sais pas moi-même|, songea-t-elle avec ironie.|>(Aislinn)

[Tu te jettes à corps perdu dans le premier mirage d’énigme que tu as pu trouver pour t’éviter de penser à la conversation que tu as eue avec le vieux schnock en début d’ennéade.](Sanaa) Aislinn se décomposa presque instantanément, avant de secouer péniblement la tête ; intraitable, Sanaa insista. [Je ne te laisserai pas lui céder, Ellie. Pas *sur ça*. Tu mérites mieux.](Sanaa)

[Rien n’est encore fait,| protesta faiblement l’héritière Noblegriffon en levant ses paumes devant elle.| Je n’ai cédé à rien, j’ai juste dit que j’y *réfléchirai*.](Aislinn)

[Chez toi, c’est une manière d’annoncer que tu vas céder,| s’entêta Sanaa en croisant ses bras sur sa poitrine.| Tu en as parlé à Digne ?](Sanaa)

[Pas encore…| souffla Aislinn en détournant le regard.| Je ne m’explique pas pourquoi tu insistes tant à ce que je le fasse, d’ailleurs.](Aislinn)

[Tu ne crois pas que ça risque de l’intéresser ?](Sanaa)

[Je ne vois pas pourquoi,| objecta la Noblegriffon un peu piteusement en haussant les épaules.| Pour toi, encore, je pourrai comprendre…](Aislinn)

Et la Vaanie de soulever un sourcil surpris. Se rendant compte qu’elle venait de commettre *la* bévue qu’elle cherchait désespérément à éviter depuis que ses deux amis se fréquentaient assidûment, elle eut un mouvement de recul, avant de faire la seule chose qui lui traversa l’esprit en cet instant précis : mimer de passer à autre chose. Elle tendit sa dextre pour se saisir des mémoires de la gardienne de Tyra, mais Sanaa fut plus rapide qu’elle et posa la paume de sa sénestre dessus.

[Comment ça, *pour moi* ?](Sanaa)

[Eh bien, c’est-à-dire que…| commença Aislinn avant de rendre les armes.| Tu ne vas pas me faire croire que tu pensais avoir été *discrète* ? Tu es allé le retrouver chez lui un nombre incalculable de fois, ça me paraît évident que… Pourquoi tu souris ?](Aislinn)

[Je ne souris pas,| la détrompa sa cadette.| Je me mords les joues très fort pour ne pas éclater de rire.](Sanaa) Elle contredit ses paroles le temps de s’esclaffer. [Mais c’est pour ça que tu agis comme une nigaude depuis deux ennéades !](Sanaa) Elle se leva enfin de son fauteuil, sa surprise passée, avant de lui saisir les mots et d’annoncer, les prunelles pétillant de malices : [Il n’y a rien de tout cela entre Digne et moi, Ellie.](Sanaa)

Et la Rivoise de répondre, non sans sentir son cœur manquer un battement : [Oh ?](Aislinn)

_________________date_________________________________
Dans la soirée de l’Elenwënas 41 Verimios de l’An `13:XI`, dans une taverne de Thaar.
_____________________________________________________

Aislinn et Sanaa avaient continué leur discussion tout le reste de la journée, au grand soulagement de la première qui retrouvait par la même occasion sa confidente. Elle put ainsi s’ouvrir de toutes les émotions ambivalentes qu’avaient pu provoquer l’attitude de Digne à son égard et la Vaanie, pour ça part, y avait été de quelques révélations sur les sentiments du jeune Hadjaoui. Il tenait du miracle que, le soir venu, l’héritière Noblegriffon eût réussi à donner le change et ne rien laisser paraître de sa confusion.

[Je suis loin d’être la pire,| répondit-elle avec bonne humeur et légèreté.| L’une des matrones qui m’a élevé s’appelait  *Chobanne*, qui s’écrit s-i-o-b-h-á-n.](Aislinn)

Se disant, elle remarqua le coup d’œil circulaire de Digne autour de lui et comprit à son tour qu’ils étaient désormais seuls… et que les doigts de sa dextre s’étaient — l’alcool aidant — mêlés à ceux de la sénestre du thaari. Elle retira vivement son bras, comme si leur contact la brûlait, avant de bredouiller des excuses en baissant le regard, les joues en feu.

L’idée que Sanaa et Aimé décidassent de revenir à ce moment-là lui fit horreur et elle se leva d’un coup de sa chaise.

[On sort prendre l’air ?| demanda-t-elle en se tournant déjà vers la porte.| J’étouffe.](Aislinn)

Et, pour mieux le convaincre, elle tendit sa main droite dans sa direction, sans être vraiment certaine qu’elle espérait qu’il la saisît ou non.

Elle avait quand même sa petite idée.
___________________personnage__________________
                             Aislinn Noblegriffon
_____________________________________________

[Juste le temps de souffler un peu,| acquiesça la Rivoise en mêlant plus encore ses doigts à ceux du jeune homme.| Je suis désolée, c’est venu d’un coup.](Aislinn) Elle pouvait blâmer Sanaa pour cela ; après avoir insisté des jours durant pour qu’elle confiât à Digne sur les projets de Varlar, la Vaanie la mettait purement et simplement au pied du mur en s’éclipsant ainsi avec Aimé. <Si je ne dis rien ce soir, elle le fera à ma place|, savait-elle pertinemment.|>(Aislinn) Aislinn n’avait aucun mal à imaginer sous quel angle l’adolescente présenterait cette confession et Digne n’avait vraiment pas besoin de cela.

Ils marchèrent silencieusement quelques minutes qui parurent des heures à la Noblegriffon, dont la seule véritable interaction avec le thaari consistait à caresser inconsciemment le dos de sa main avec son pouce. Pour le reste, elle était trop concentrée sur les aveux qu’elle s’apprêtait à faire pour tenir convenablement une conversation. Elle finit par arriver à la conclusion qu’elle ne parviendrait pas à formuler le nœud du problème autrement qu’abruptement et décida de ne plus se torturer à essayer. S’arrêtant tout net, elle se tourna vers l’ancien esclave, l’invita à faire de même, puis darda des prunelles résolues sur le visage inquiet de Digne.

[Varlar veut organiser mon mariage,| expliqua-t-elle d’une voix tendue.| Il pense que c’est une étape obligée pour préparer notre retour en Péninsule.](Aislinn)

Elle sentit que, sous le coup de la surprise, son interlocuteur cherchait déjà à se dégager ; elle l’en empêcha en attrapant sa seconde main pour mieux le rassurer.

[Laisse-moi finir,| lui intima-t-elle d’une voix douce.| Je savais que cela pouvait arriver ; j’ai vite compris, après que Mère m’a reconnue comme sa fille, que je venais d’entrer malgré moi dans un monde qui n’était pas le mien. Je suis son héritière, Digne. L’héritière des Noblegriffon. Par delà l’Olienne, dans le Royaume de Diantra, plusieurs comptoirs marchands fondés par Mère continuent d’utiliser son nom. Il espère parvenir à les récupérer en se servant de moi pour légitimer ses revendications… et du mariage qu’il ambitionne sceller pour moi pour se forger une alliance avec une grande famille de la Péninsule.](Aislinn)

Elle voyait qu’il ne comprenait pas, mais elle ne pouvait pas lui en vouloir. Elle perdait beaucoup trop de temps à se disperser dans des détails inutiles, qui ne pouvait que le troubler un peu plus quand tout ce qu’elle souhaitait été qu’il saisît bien toutes les implications de ce qu’elle cherchait à lui dire.

[Il y a quelques ennéades encore, j’étais prête à faire ces sacrifices pour le nom de Mère, mais je sais maintenant que c’était une erreur. Sous couvert de veiller à mes intérêts et ceux de ma lignée, Varlar sert en réalité ses propres desseins. Je l’ai trop longtemps laissé libre de faire ce qu’il voulait sans rien remettre en question, mais c’est terminé. J’ai ouvert les yeux.](Aislinn)

Elle pourrait lui expliquer les différentes étapes de la réflexion qui l’avait amené à cette conclusion plus tard, si toutefois il lui demandait. Pour l’heure, elle préférait plutôt essayer d’aller à l’essentiel.

[Je vais lui tenir tête, Digne. Je te le promets. Je me marierai le jour où je l’aurai décidé, avec le partenaire de mon choix.](Aislinn)

Et la jeune héritière de laisser ses mains trembler en affirmant cela, car bien qu’elle était encore loin de considérer la moindre union, au moins l’idée d’un partenaire en particulier avait commencé à faire son chemin. Elle le signifia bien malgré elle, en le tirant inconsciemment vers elle.
__________________________personnage__________________________
                                              Aslinn Noblegriffon
_____________________________________________________________

<Que suis-je en train de faire, exactement ?| songeait confusément la jeune Noblegriffon lorsque les lèvres de Digne frôlaient les siennes une première fois.|>(Aislinn) Elle ne se déroba pas, cependant. Bien au contraire, elle resserra sa prise sur les doigts du Thaari et esquissa un sourire ravi tandis qu’ils s’embrassaient. Elle avait passé des jours entiers à se torturer en se demandant si elle avait envie que Digne cueillît ainsi ses lippes ; elle avait sa réponse désormais. Il s’écarta un peu, pour poser sur elle ce regard qu’elle aimait tant et elle darda sur lui des prunelles pétillantes de joie et brillantes de tendresse. Il ne dit rien et préféra se pencher une seconde fois. La jouvencelle se laissa volontiers guider, mais quand elle sentit la langue du jeune homme venir chatouiller ses lèvres, elle se rendit compte qu’ils s’aventuraient *déjà* en dehors de sa zone de confort; elle entrouvrit légèrement la bouche pour l’accueillir, mais demeura quant à elle bien timorée. <Il doit me trouver gourde|, regretta-t-elle tandis qu’ils se séparaient à nouveau et plutôt que d’affronter son regard, elle nicha son nez dans son cou.|>(Aislinn)

Elle aimait être dans ses bras. Tout y était plus simple, pour un temps du moins. Elle savait, bien sûr, que la réalité l’attendait toujours de pied ferme, mais elle gagnait en se lovant contre lui quelques précieuses secondes où elle pouvait oublier tous ses tracas. Désireuse de se rattraper après ce qu’elle jugeait déjà être une bien piètre prestation, elle déposa quelques doux baisers dans le cou du jeune homme ; elle avait des femmes agir ainsi avec leurs compagnons et ces derniers avaient semblé apprécier. Elle avait d’un coup tant et tant de questions et personne qu’elle pouvait interroger ; cela lui donna le vertige, mais elle ne voulait pas céder à la panique. Pas alors qu’elle s’était senti si bien, quelques secondes à peine auparavant. Elle refusait que sa peur de ce qui pouvait arriver lui gâchât son instant présent.

[Je suis heureuse,| énonça-t-elle en écho aux paroles de Digne,| mais même si c’est difficile à croire, je pense que de nous trois c’est Sanaa qui jubilera le plus.](Aislinn) Elle rit malgré elle, de ce rire clair et joyeux dont elle avait le secret. [Oh, Digne,| expia-t-elle en se blottissant à nouveau contre son torse et son souffle chaud glissa sur son cou à lui.| Comme tu as été patient avec moi.](Aislinn)

Elle avait l’étrange impression que la nature de leur relation changeait à toute vitesse depuis leur premier baiser et cela la grisait ; elle n’avait par ailleurs aucune envie de retourner dans l’auberge, mais n’ignorait pas que s’ils ne le faisaient pas avant longtemps, Aimé et Sanaa finiraient par s’inquiéter. Se tournant dans la direction de la bâtisse qu’ils avaient quitté sans un mot pour leurs êtres chers, elle esquissa une légère moue.

[Je ne sais pas quoi faire,| avoua-t-elle à Digne.|](Aislinn) <Je ne sais pas quoi, ni comment leur dire|, aurait-elle dû préciser, mais elle garda pour elle cette seconde confession.|>(Aislinn)
______________________personnage________________________
                                        Aislinn Noblegriffon
______________________________________________________

Aislinn darda sur Digne des prunelles où la joie disputait clairement à la confusion une place d’honneur. Elle avait l’impression d’être perchée sur un petit nuage, et dans le même temps de se tenir face à un précipice. Sans doute le jeune Hadjaoui était-il sincère dans sa démarche et ne désirait rien d’autre que de bien faire, mais assurément l’héritière Noblegriffon aurait voulu qu’il attendît un petit peu avant d’aborder ce genre de questions avec elle. Ce n’était pas tant qu’elle ne souhaitait pas être honnête avec lui ou lui cacher ses sentiments, mais plutôt qu’elle n’avait encore aucune véritable idée de ce qu’elle ressentait !

Plutôt que de lui expliquer cela, elle prit le parti d’essayer une approche «légèrement» différente. Sans parvenir à retenir un sourire timide, elle se lova à nouveau contre lui, passa ses bras autour de son cou et l’attira vers elle pour l’embrasser ; les lèvres du jeune homme sur les siennes lui firent l’effet d’une décharge électrique et, pour sceller définitivement l’inversion — au moins temporaire — de leur rôle, ce fut sa langue à elle qui chercha cette fois à se frayer un chemin entre ses lippes à lui.

Ce troisième baiser fut plus long et passionné — mais certainement tout aussi maladroit — que ces deux prédécesseurs et provoqua chez Aislinn un bien étrange et délicieux mélange de sensations auxquelles elle n’était décidément pas familière. Quand ils se séparèrent, elle avait le souffle court et ses joues roses.

[Je sais que tu respecteras mes choix,| lui répondit-elle enfin.| Je respecterai les tiens.](Aislinn)

Elle avait prononcé ces paroles le cœur vibrant de sincérité, mais sa promesse à peine formulée, son esprit se joua d’elle et elle se rappela que le père Hadjaoui avait épousé non pas une, mais bien trois femmes. L’adolescente, dont le sang était toujours chargé de l’adrénaline de leur baiser, fut saisi d’un bien désagréable malaise en visualisant Digne en embrasser une autre. <Pas ce soir|, s’admonesta-t-elle silencieusement.| Vite. Très vite. Juste pas ce soir.>(Aislinn) Elle se découvrait *amoureuse*, mais n’était pas prête à s’imaginer *possessive*.

Sa résolution fut prise en un fragment de seconde à peine et le Thaari put tout aussi bien ne pas remarquer son débat interne prestement étouffé et quand bien même il l’avait pu, le sourire sincère et éclatant de la jouvencelle devait l’aider à ne plus y penser.

[Je sais que je vous ai promis de ne pas venir trop souvent à l’auberge pour ne pas vous déranger quand vous travaillez, mais je doute désormais réussir à tenir parole encore très longtemps…| annonça-t-elle avec un brin de malice et beaucoup d’honnêteté tandis que son cœur s’emballait à nouveau.|](Aislinn)
________________________personnage______________________
                                             Aislinn Noblegriffon
______________________________________________________

___________________________date________________________
Elenwënas 50 Verimios de l’An `13:XI`
______________________________________________________


Le nouveau couple mit les ennéades qui suivirent à profit pour trouver — l’un comme l’autre et l’un avec l’autre — leurs marques. Pour sa part, Aislinn eut besoin de quelques jours pour prendre toute la mesure du soudain — et néanmoins espéré — tournant de sa relation avec le jeune Hadjaoui. Elle retourna dans tous les sens le déroulement des événements de la soirée, de ses confessions sur les projets condamnables de Varlar jusqu’à leur promesse de se revoir très vite, en passant bien sûr par leur troisième baiser passionné dont elle avait été à l’initiative. La Noblegriffon demeurait confuse encore en repensant à la hardiesse dont elle avait fait preuve, bien qu’elle pouvait l’expliquer sans mal. [Je ne veux pas être celle que Digne passe son temps à attendre,| avait-elle résumé à Sanaa.|](Aislinn) Et de garder par-devers elle l’autre versant de l’histoire : <Je ne veux pas être celle dont il se lasse et ne se contente plus.>(Aislinn) Elle avait beau vivre en Estrévent depuis trois longues années, la polygamie de certains vaanis continuait de la rendre perplexe et dans les balbutiements de son idylle, elle n’avait pas pu s’empêcher de trouver dans cette étrange pratique une forme d’épée de Damoclès.

La prévenance, la patience et la gentillesse de Digne eurent cependant rapidement raison de ses craintes et elle chassa finalement bien vite ses angoisses pour mieux profiter des attentions du jeune homme… et apprendre à en prodiguer, à son rythme et sans contrainte. De ses atermoiements des premiers jours, pourtant, elle ne dit rien au premier concerné — ou très peu —, quand bien même il l’y avait encouragé dès le premier soir et recommença plusieurs fois par la suite. Toute sa vie durant, Aislinn avait intériorisé ses ressentis, faute d’interlocuteurs à qui se confier. Ainsi à Katalina, elle avait avoué bien peu de choses, eu égard à tout ce qu’elle avait pu éprouver tout le temps qu’elle était restée à ses côtés. Sanaa était la première personne auprès de laquelle elle avait commencé à s’ouvrir et même à elle, les mots ne lui venaient pas toujours naturellement. Il n’y avait qu’à constater les longues ennéades durant lesquelles elle avait tû ses sentiments naissants pour Digne.

[Le problème d’Aislinn, c’est que lorsque quelqu’un la frappe, sa première pensée est de se demander si son gourdin n’est pas trop lourd pour lui,| avait un jour doctement — mais peut-être un peu maladroitement aussi — expliqué Sanaa à l’affranchi.|](Sanaa) La Rivoise avait, c’était un fait, une capacité de résilience sans commune mesure avec la plupart des adolescents de son âge et une tendance quasiment maladive à mettre le bien être de ses êtres chers avant le sien propre. Entre les lignes, elle avait essayé de prévenir Digne contre les cercles vicieux que cela pouvait engendrer… De son point de vue inavoué, la relation de sa meilleure amie avec sa mère adoptive était un parfait et tragique exemple de ce à quoi la retenue d’Aislinn pouvait mener.

Cet avertissement de mauvais augure mis à part, Digne avait peu de raisons de se faire du souci, car avant longtemps, Aislinn commença à vraiment s’épanouir à ses côtés. Elle s’invita de plus en plus souvent à l’auberge, « volant » au tavernier son employé chaque fois qu’elle le pouvait. Elle insista pour qu’ils continuassent son initiation à l’escrime. Elle le tint, aussi, au courant de ses «progrès» sur la question de son mariage, encouragé en cela par Sanaa, qui avait fini par lui faire comprendre que, non, cela n’avait aucune chance de le *déranger*. Aislinn en était venu à hautement apprécier ces moments d’échanges, durant lesquels le Hadjaoui la canalisait et l’aidait à mettre un peu d’ordre dans ses pensées.

En l’occurrence, Varlar n’avait pas semblé particulièrement pressé de mener ses projets à bien. Tout juste lui en parlait-il de temps en temps. Il ignorait tout de l’idylle naissante de sa protégée et ce sursis laissait à cette dernière du temps pour organiser sa riposte ; pour l’heure, elle faisait son possible pour prendre au mieux la mesure de la véritable nature des comptoirs qui portaient le nom de sa mère de ce côté-ci de l’Olienne. Une tâche plus conséquente qu’elle ne l’avait considérée de prime abord.

Une trentaine de jours après leur premier baiser, Aislinn et Digne s’entraînaient comme très souvent dans l’arrière-cour de l’auberge où le jeune homme et son aîné travaillaient. Cette fois-ci, contrairement à son habitude, la Rivoise se révéla distraite et peu motivée. Ce fut sans doute ce qui poussa Digne à couper court à leurs exercices. Il la connaissait assez bien désormais pour deviner quand elle *voulait* lui annoncer quelque chose, mais tournait son discours dans sa tête sans parvenir à lui faire franchir la frontière de ses lèvres.

Ils rangèrent leurs armes factices, puis allèrent s’asseoir sur «leur» muret. Avant longtemps, Aislinn posa sa tempe contre l’épaule du jeune homme. Les yeux fermés, elle se concentra sur sa respiration.

[J’ai un service à te demander,| finit-elle par lui annoncer en se redressant pour planter son regard dans le sien.| Ou plutôt, *j’aimerais bien* ne pas avoir à te le demander, mais je crois que sans ton aide, je n’y arriverai pas.](Aislinn)

Elle dut se rendre compte elle-même qu’elle recommençait à présenter les choses dans le mauvais ordre, car elle esquissa un sourire complice à l’attention de son compagnon, avant de reprendre d’elle-même son explication par le bon bout.

[Varlar est certain de ma docile coopération dans ses projets concernant mon futur mari. Je voulais faire profil bas afin de me laisser l’opportunité d’en apprendre un peu plus, mais plus le temps passe, plus je risque de me retrouver acculée et rattrapée par mon propre piège. Je ne dois pas oublier que de nous deux, il reste et restera le plus roublard.](Aislinn) Elle marqua une courte pause, puis entra enfin dans le vif du sujet : [Il a réussi à nous faire inviter dans une des nombreuses célébrations qui commencent à s’organiser pour fêter le début imminent des grands jeux et j’ai découvert qu’il prévoit de me présenter à mon… eh bien, à celui qu’il croit être mon futur époux. J’ai *besoin* de savoir qui est cet homme et de le rencontrer, ne serait-ce que pour lui expliquer que je ne suis absolument pas… disposée ? À m’unir à lui.](Aislinn)

Elle baissa les yeux, un peu honteuse. Ayant déjà eu la «chance» d’assister à plusieurs événements mondains des hautes sociétés de Thaar, elle ne pouvait pas envisager que sa demande à venir pourrait causer autre chose à Digne que du tracas.

[J’imagine que tu as tout sauf envie de te retrouver dans ce genre d’endroits, mais je dois t’avouer que ça me ferait très plaisir si tu acceptais de m’accompagner.](Aislinn) Elle sourit faiblement. [Ce sera l’occasion d’annoncer la bonne nouvelle à Varlar.](Aislinn)
_______________personnage___________
Aislinn Noblegriffon
___________________________________

___________date_______________
Arkuisa 5 Barkios de l’An `13:XI`, à la recherche d’Aimé Hadjaoui
_____________________________

[Dire que l’on pourrait être dans les Soieries,| pesta à nouveau Sanaa en faisant de son mieux pour suivre le rythme d’Aislinn|](Sanaa)

[Au lieu de quoi, te voilà à écumer les mauvaises rues de la ville la plus sale du monde,| abonda l’héritière en levant les yeux au ciel.| Au moins !](Aislinn)

[Je sais que tu te moques de moi, mais je n’en démerderai pas. Thaar est sale comme... Comme...](Sanaa) L’Estréventine préféra à l’insulte qui lui chatouillait la gorge un râle exaspéré. [Tu pourrais m’attendre, tout de même. J’ai certes accepté de venir, mais certainement pas pour trottiner derrière.](Sanaa)

La Rivoise poussa un soupir devant tant de mauvaise foi, mais fut bien obligée de reconnaître que les récriminations de son amie n’étaient pas totalement infondées. Elle cessa donc sa course folle le temps pour la Vaanie de la rattraper, puis elle lui saisit les épaules et darda sur elle des prunelles déterminées.

[Tu as *insisté* pour venir,| la corrigea-t-elle en fronçant les sourcils,| et je me souviens très bien avoir posé une condition à mon accord.](Aislinn) Elle marqua une légère pause, défiant du regard son interlocutrice de la contredire ; quand il fut clair que Sanaa n’en ferait rien, elle raffermit sa prise. [Ana !| insista-t-elle en trépignant sur place.| Toi plus que quiconque sait que c’est important pour moi.](Aislinn)

La Rivoise avait envie de secouer la Vaanie jusqu’à l’entendre demander grâce, mais la sagesse lui dicta tout autrement de la libérer. Les deux adolescentes se toisèrent quelques instants, puis Sanaa rendit les armes et opina lentement du chef. [Bien sûr, que je le sais,| convint-elle avec sérieux cette fois.|](Sanaa)

[Les Noblegriffon règlent leurs dettes,| renchérit tout de même Aislinn en recommençant à marcher.| J’ai une dette envers Aimé, je vais donc la régler. Ce soir. C’est aussi simple que cela.](Aislinn) <Parce que je suis la fille de ma mère|, se remémora-t-elle silencieusement comme à chaque fois qu’elle usait de «son» patronyme.|>(Aislinn) Deux ans avaient passé, depuis que Katalina l’avait reconnue comme son héritière. Deux longues années qui n’avaient pas été de trop pour qu’elle s’habituât à son nouveau nom.

[Je suis certaine qu’il se souvient de toi,| voulut la rassurer Sanaa car c’était effectivement une des craintes de la Péninsulaire.|](Sanaa)

[Si ce n’est pas le cas, je ferai en sorte de lui rafraîchir la mémoire,| répondit Aislin avec une légèreté feinte.|](Aislinn) Elle se demandait aussi s’il la reconnaîtrait sans qu’elle eût besoin de se présenter. C’était qu’elle avait bien changé, depuis sa percutante rencontre avec Aimé dans l’Agora d’Ys. L’enfant perdue d’alors avait laissé sa place à une jeune pucelle d’une quinzaine d’années; ses cheveux avaient légèrement blondi sous le soleil d’Ithri’Vaan; sa chiche robe avait disparu, remplacée par un vêtement d’une bien meilleure fracture, qui marquait clairement son nouveau statut social… et ses formes naissantes. Elle était une héritière, désormais ; son nom était celui d’une illustre famille péninsulaire, dont les origines remontaient à plusieurs siècles. Elle était riche ; à tout le moins, c’était ce que lui répétait sans arrêt Varlar, mais c’était lui qui tenait les cordons de sa bourse encore, aussi ne pouvait-elle pas en jurer.

<Peut-être ne voudra-t-il pas me parler|, se surprit-elle à penser en s’engouffrant sans réfléchir dans une rue adjacente à celle qu’elles suivaient depuis plusieurs minutes déjà.| S’il apprend la vérité sur Sanaa, c’est possible.>(Aislinn) Il lui avait avoué, pendant le repas qu’ils avaient partagé ensemble, que certains de ses frères étaient des esclaves qui se battaient tous les jours pour regagner leur liberté. Légalement, Sanaa aussi était encore asservie. L’Estréventine, originaire des Côtes Brûlées à l’est d’Ys, avait été achetée par Varlar ; le Langecin, trouvant sa protégée trop souvent seule, la lui avait offerte dans les premières ennéades de l’année précédente. Aislinn avait été horrifiée et quand elle s’était retrouvée en tête à tête avec la Vaanie, elle lui avait promis que jamais elle ne lui donnerait le moindre ordre. Elle avait aussi juré de l’affranchir dès qu’elle en aurait la possibilité, c’était à dire à sa majorité Il n’avait pas fallu très longtemps aux deux adolescentes pour devenir inséparables, mais toujours flottait entre elles le véritable statut de Sanaa.

Il était cependant peu probable qu’Aimé le découvrît jamais, car Sanaa portait ce jour là --- et la plupart des autres, en vérité --- peu ou prou la même tenue qu’Aislinn.

[Ellie ! je crois que nous sommes arrivées,| annonça la Vaanie en indiquant une auberge de sa dextre.| Tu penses que c’est la bonne, cette fois ?](Sanaa)

[Je l’espère sincèrement...| souffla Aislinn en sentant son ventre se nouer.|](Aislinn) Deux ans avaient passé, depuis qu’Aimé lui avait sommairement expliqué son travail. Elle n’avait jamais oublié le nom de l’auberge qui l’employait, mais Thaar était si grande ! Il avait fallut qu’elle y cherchât quelque chose dont elle savait trop peu de choses pour se rendre compte d’à quel point.

[Ne t’inquiète pas, si ce n’est pas le cas, nous n’aurons qu’à continuer à fouiller la ville,| affirma Sanaa en lui adressant un sourire radieux.| Il reste bien assez de temps avant le crépuscule pour ça.](Sanaa)

Aislinn lui adressa un clin d’œil de remerciement, inspira profondément, puis poussa la porte d’entrée de l’auberge --- la troisième qu’elle visitait ce jour-là, et la cinquième en comptant celles de la veille.
_______________________personnage_______________________
                                            Aislinn Noblegriffon
______________________________________________________

[Je ne le laisserai pas s’en prendre à toi,| répondit très vite Aislinn à Digne lorsqu’il évoqua la réaction possible de Varlar quand le vieux Langecin le rencontrerait.|](Aislinn)

Elle avait longuement hésité avant de se décider à l’inviter à se présenter à son bras à cette fameuse soirée ; elle n’était toujours pas persuadée qu’elle agissait de la manière la plus sage qui fût et même le sourire du jeune Hadjaoui ne parvint pas à chasser complètement ses craintes. Elle mêla ses mains aux siennes avec tendresse et commença à masser doucement ses paumes avec ses pouces.

[Je ne laisserai personne toucher le moindre de tes cheveux,| lui promit-elle avec un aplomb, une détermination et une sincérité que Digne n’avait peut-être pas anticipé.|](Aislinn)

[Je viendrais,| lui assura-t-il et le cœur de la Rivoise se gonfla de gratitude.| Mais... Je pense que tu as vu l'étendue complète de ma garde-robe. On risque de me prendre pour ton serviteur.](Digne)

[Je ne me fais pas trop de soucis pour ça,| répondit-elle en pouffant.|](Aislinn) L’idée qu’elle ne serait pas seule pour affronter son futur prétendant — et ses potentielles protestations — la soulageait indubitablement et cela se constatait rien qu’à la manière dont elle se tenait ; elle était d’un coup beaucoup plus détendue au niveau des épaules et bien moins voûtée. [Il est possible que j’aie déjà évoqué mon intention de t’embrigader dans cette gageure et elle n’attend plus que le moment où elle pourra t’habiller des pieds à la tête.](Aislinn)

Elle darda sur lui des prunelles où l’amusement le disputait à la contrition et elle souffla un [désolé](Aislinn) muet… avant de retourner nicher son visage dans son cou et de le serrer dans ses bras.

[Ce sera vite fini,| lui promit-elle encore.| Cette soirée, les projets de Varlar, je vais faire ce qu’il faut pour que nous n’ayons plus à nous en soucier.](Aislinn)
________________________personnage______________________
                                                         Sanaa
______________________________________________________

Sans s’en cacher, la jeune Vaanie dévora du regard le *fameux* Noble Lame, dont elle avait eu vent des exploits par le truchement des Hadjaoui. C’était la première fois qu’elle le rencontrait et elle devait avouer qu’il n’était pas tel qu’elle l’avait imaginé ; et dans le même temps, elle aurait été bien incapable de détailler en quoi.

[Tout l’enchantement est pour moi,| répondit Sanaa à Sauveur en esquissant une révérence pleine d’élégance.|](Sanaa) Elle ajouta ensuite à l’adresse d’Aimé : [Bien sûr que je veux bien de vous. Vous êtes arrivés juste à temps, ils sont sur le point de partir.](Sanaa)

Et la Vaanie de faire un pas de côté pour les inviter à ne pas rester plus sur le palier de la porte, puis de reprendre son examen du gladiateur quand ce dernier la dépassa. Elle darda ensuite un regard joyeux sur Aimé et ne fut pas surprise de déceler semblable bonne humeur dans ses prunelles pleines de sagesse. Depuis le début de la romance entre Digne et Aislinn, Sanaa avait passé un temps certain avec l’aîné des Hadjaoui et elle en était venue à beaucoup l’apprécier ; ils étaient notamment très complices dès lors qu’il s’agissait de s’extasier sur les gestes d’affection que se réservaient leurs protégés.

La demeure du tuteur d’Aislinn était par beaucoup d’aspect similaire à celle qu’il avait possédée quelques années plus tôt à Ys ; elle était aussi d’une taille comparable, ce qui en disait long sur l’enrichissement personnel du vieux Langecin, car qui pouvait le plus à Ys pouvait le moins à Thaar. La poterne d’entrée débouchait dans les couloirs d’une espèce de cloître, dont le jardin était parfaitement entretenu et d’une grande beauté. C’était pour beaucoup grâce à Aslinn, expliqua rapidement Sanaa en les guidant jusqu’à la porte principale.

[Le vieux schnock se fiche comme d’une guigne de sa demeure, tant qu’elle *semble* prospère. Ça lui coûte moins cher, à ce qu’il paraît.](Sanaa)

Aimé était habitué à entendre la Vaanie se plaindre de Varlar, mais il manquait à Sauveur beaucoup de clefs pour prendre pleinement conscience du mépris que la jeune adolescente nourrissait à l’égard du Langecin. Elle avait avoué à Digne que, au moins légalement, elle lui *appartenait* encore, comme toutes les possessions d’Aislinn et ce jusqu’au seizième anniversaire de l’héritière. La façon dont le benjamin Hadjaoui avait accueilli la nouvelle l’avait beaucoup aidée à trouver un peu de paix intérieure sur cette question et elle avait fini par mettre Aimé dans la confidence à son tour. Quant à Sauveur, elle doutait très fortement qu’il fût au courant, mais elle n’ignorait pas que lui aussi était asservi à quelqu’un et tandis qu’elle posait sur lui un regard amusé par sa propre tirade, elle prit conscience qu’ils étaient peut-être plus similaires qu’elle n’avait pu le penser de prime abord.

À lui comme à elle, on voulait faire croire que ses chaînes étaient invisibles et légères, mais il devait savoir qu’il n’en était rien et qu’à tout moment, la daedhelle qui le possédait pouvait tirer violemment sur ses liens pour le ramener « à sa place ». C’était quelque chose que Sanaa redoutait plus que tout — que Varlar finît par se lasser et décidât de la revendre avant la date fatidique —, même si elle refusait de se laisser gouverner par cette crainte.

<À moi, cependant, on ne demande pas de me battre en risquant ma vie|, songea-t-elle pour nuancer ses réflexions.|>(Sanaa)

Elle les guida jusqu’au cœur de la demeure : un ensemble de pièces construites autour d’un grand salon où Varlar pouvait rencontrer ses partenaires commerciaux dans de bonnes dispositions. C’était là-bas que Digne et Aislinn continuaient de discuter de la soirée à venir.

Juste avant de pousser l’ultime porte qui les séparait des deux tourtereaux, Sanaa se figea et esquissa une grimace mi-figue mi-raisin en se tournant vers Aimé.

[Je parie tout ce que tu veux qu’en découvrant la présence de Sauveur, Aislinn va proposer à Digne de tout annuler pour qu’il puisse profiter de son frère.](Sanaa)

Elle reporta son attention sur le principal concerné. Le regard toujours un peu sombre du gladiateur l’empêcha de poursuivre. <Je ne peux pas lui demander de promettre à Digne qu’ils auront d’autres occasions de passer une soirée ensemble|, avait-elle réalisé juste à temps.|>(Sanaa)

Et l’adolescente de couper court à cet interlude en ouvrant en grand la porte… pour découvrir Digne et Aislinn enlacés si étroitement qu’un assassin n’aurait pas pu glisser entre eux la lame de sa dague, en train d’échanger un long et passionné baiser.
______________________personnage________________________
                                          Aislinn Noblegriffon
______________________________________________________

Qu’Aislinn eût été capable de retenir le petit cri de surprise qui lui avait chatouillé la gorge tenait du miracle. La jeune femme s’écarta de Digne rouge comme une pivoine et bredouilla quelques excuses inaudibles, sans pour autant lâcher la main de l’Hadjaoui avec qui elle échangeait un baiser passionné quelques secondes plus tôt.

[Sauveur?...| réagit Digne en premier et elle darda sur lui des prunelles étonnées, avant de porter son attention sur le gladiateur.|](Digne)

Laissant son compagnon profiter de ses retrouvailles avec son frère, la Rivoise occupa ses doigts en essayant de remettre un peu d’ordre dans une coiffe qui n’en avait pas particulièrement besoin. Ses cheveux étaient nattés serrés au niveau des tempes, puis attachés en une demie-queue au sommet de son crâne. À partir de cette base simple, mais efficace, Sanaa avait fait des merveilles en agrémentant ça et là les mèches blondes de son amie avec des perles et des pierres semi-précieuses dont les couleurs froides étaient autant de rappels du vert pâle de sa robe. Elle avait refusé qu’on l’affublât de ces « morceaux de tissus qui ne couvraient rien, ne suggéraient pas grand-chose et laissaient voir tout ce qu’elle ne voulait pas montrer » et son vêtement était donc relativement sage, eu égard à ce que les Vaanies abordaient volontiers. Ses principales audaces résidaient dans l’échancrée de son dos, bien plus prononcé que tout ce qu’elle avait pu porter jusqu’alors, ainsi que ses bras pour grande partie nus.  Elle avait concédé cette fantaisie à Sanaa qu’à la condition qu’elle pût l’accompagner d’un long châle diaphane… qu’elle n’arrêtait pas de remettre en place depuis le moment où on le lui avait installé sur les épaules.

Elle reporta son attention sur les Hadjaoui quand Sauveur le complimenta sur sa tenue ; Aislinn eut très envie d’abonder dans le sens du gladiateur, car il était vrai que Digne était resplendissant dans sa toge immaculée, mais quelque chose dans l’attitude de Sauveur l’en empêcha.

Elle savait qu’en son temps, Aimé lui aussi avait été obligé de se battre dans l’arène, mais aux yeux d’Aislinn, il avait toujours dégagé une présence rassurante ; impressionnante, mais apaisante. Depuis qu’elle avait posé les yeux sur Sauveur, la Rivoise ne pouvait qu’être frappée par le contraste qu’offraient les deux frères. Et de se demander si, du temps où il maniait l’épée pour survivre, Aimé inspirait quelque chose de similaire à la rage contenue de son cadet.

<Si seulement je pouvais faire quelque chose|, regretta-t-elle à nouveau.| Si tout cet argent que je suis censé posséder pouvait servir à autre chose qu’à en amasser toujours plus.>(Aislinn) Malheureusement, elle savait Sauveur jouir des faveurs de la Princesse Marchande Maralina Irohivrah ; il était impensable qu’elle parvînt à convaincre Varlar d’aligner sur la table assez d’or pour que la maîtresse de Uldal'Rhiz acceptât de renoncer à son jouet. Cette idée alimentait sa colère, car elle résumait tout ce qu’elle détestait de Thaar.

Quand il lui adressa directement la parole, elle croisa son regard et son malaise décrut légèrement. Ses yeux étaient durs, mais elle pouvait déceler dans ses prunelles une joie sincère pour le bonheur naissant de son benjamin. Cela la rassura un peu. Elle allait pour lui répondre quelque chose, mais Digne fut le plus rapide et elle ne lui en tint aucunement rigueur. Elle s’avança pour les rejoindre, mais demeura en retrait tout de même, pour ne pas s’imposer plus que de nécessaire.

La petite phrase du gladiateur provoqua en elle un mélange de sentiments qui n’était pas si éloigné de ce que pouvait ressentir son « amoureux », ainsi que l’avait vocalisé Sauveur avec une simplicité désarmante.

[Bien sûr que je suis une excellente hôtesse,| abonda un peu crânement Sanaa au dernier échange des Hadjaoui.| ](Sanaa)

[Digne m’a énormément parlé de toi,| put-elle enfin répondre aux salutations de Sauveur.| Je suis heureuse de pouvoir te rencontrer et je te promets de prendre soin de ton frère tout le temps qu’il sera à mes côtés.](Aislinn)

Elle pensait en premier lieu à la soirée vers laquelle ils se dirigeaient — elle ne parvenait pas à se départir du sentiment de l’emmener tout droit dans un panier de crabes —, mais pas seulement. Ainsi que Digne avait pu s’en rendre compte désormais, Aislinn avait un instinct de protection très développé.

[En vrai, Digne, si tu veux rester ici pendant que je…| commença-t-elle avant d’être coupée pas son amie.|](Aislinn)

[Ah ah !| triompha-t-elle en se tournant vers Aimé.| Je le *savais*. Je la connais comme si je l’avais faite.](Sanaa)

Confuse, Aislinn darda un regard empourpré et un brin accusateur sur la Vaanie, qui lui décocha son plus beau sourire en retour.

[Je voulais juste dire que je comprendrais si c’était sa volonté, c’est tout…| protesta-t-elle en s’en remettant au principal concerné pour trancher une bonne fois pour toutes la question.|](Aislinn)

Pour sa part, elle savait qu’elle ne pouvait plus se dérober et elle le lui avait assez répété pendant qu’ils se préparaient.
______________________personnage________________________
                                                    Sanaa
______________________________________________________

[Oula, surtout pas !| prostesta Sanaa en levant les bras devant elle.| Aislinn m’a raconté en détail ce qu’est *vraiment* un chevalier et, crois-moi sur parole, tu vaux bien mieux que ça.](Sanaa) Elle darda un regard un brin provocateur sur la gladiateur avant d’enchaîner : [Et je ne connais pas encore très bien Sauveur, mais je suis certaine que c’est aussi ton cas.](Sanaa) Elle fit quelques pas en direction du centre de la pièce et commença à tourner lentement sur elle-même en continuant gaiement : [Bref, bienvenue chez le vieux schnock. Si vous cassez quelque chose, faites en sorte que ce soit quelque chose de cher.](Sanaa)

Elle tendit un bras en direction de trois fauteuils disposés autour d’une table basse sur laquelle on avait fait installer trois verres, autant de bouteilles et quelques amuse-bouches.

[Les serviteurs de Varlar ont eu la gentillesse de nous préparer de quoi nous sustentés. Le plat principal — un tajine de légume — est encore dans la cuisine, il faudra aller le chercher un peu plus tard,| expliqua-t-elle.| Ils sont partis, désormais, on ne pourra donc pas les remercier. Le vieux refuse de les laisser occuper une des chambres de sa demeure, il dit que ça lui fait faire des économies.](Sanaa)

Elle marqua une pause, rattrapée par une évidence ; elle pivota à nouveau sur ses pieds pour faire face aux deux frères et leur offrit une petite moue contrite.

[Aimé commence à avoir un peu l’habitude, mais je sais que j’ai tendance à… hum. M’acharner sur ce pauvre Varlou.](Sanaa) Elle haussa les épaules. [Je vais écouter le conseil que m’a un jour donné un grand sage |— elle fit un clin d’œil à Aimé —| et ne plus penser à sa vilaine trogne à partir de… maintenant !](Sanaa)

Elle fit quelques pas de côtés, puis s’installa confortablement dans l’un des fameux fauteuils qu’elle avait désignés quelques secondes plus tôt. Elle ramena ses jambes sous elle avec simplicité, puis se lova dans les coussins qui bordaient le dossier et les accoudoirs de son siège.

[Vous avez vu comment ils étaient gênés ?| enchaîna-t-elle quand elle eut trouvé une position qui lui convenait.| C’était tellement mignon.](Sanaa) Et la Vaanie d’esquisser un sourire de connivence à Aimé, avant de darder un regard plus ambigu sur Sauveur. Si Aislinn avait été impressionnée par le gladiateur, Sanaa, elle, se découvrait fascinée par ce sombre personnage. Elle n’avait qu’une envie qu’il parlât de lui, de sa vie, de ses combats, de ses victoires aussi. Il voulait qu’il lui expliquât comment il gardait la tête haute face à ceux qui lui faire ployer l’échine. Et dans le même temps, elle *savait* que c’était la pire des choses à faire.

Assurément, il espérait profiter de sa soirée sans qu’une adolescente trop curieuse le ramenât sans cesse dans cette arène à laquelle il cherchait à échapper.

______________________personnage________________________
                                          Aislinn Noblegriffon
______________________________________________________

__________________________date_________________________
Dans une villa des Soieries appartenant à un riche marchand proche du Conseil.
______________________________________________________

Aislinn et Digne ne mirent pas beaucoup de temps à parcourir la distance qui séparait la demeure de Varlar de la villa où se tenait la fameuse fête durant laquelle le destin d’Aislinn était appelé à se nouer… ou, si elle parvenait à ses fins, à se dénouer. Les quelques mercenaires chargés par le propriétaire des lieux ne les retinrent pas et ils purent se mêler aux convives sans plus de peine.

Plusieurs choses devaient s’imposer à eux, avec des degrés de violence variés.

D’abord, ils venaient clairement de pénétrer dans un autre monde. Même Aislinn, qui vivait dans un confort certain depuis que Varlar était devenu son tuteur, n’avait jamais été confrontée à quoique ce fût de similaire.  Tout transpirait l’opulence. La nourriture qui débordait des plats, les plats eux-mêmes qui étaient en argent pour la plupart. Les vêtements des invités rivalisaient de tissus et de métaux hors de prix. Une courtisane leur coupa la route : elle portait tellement peu de tissu qu’Aislinn put bien malgré elle entrevoir son intimité, mais les étoffes dont était faite sa robe étaient entrelacées avec des fils d’or purs. <La donnerait-elle à Digne, il pourrait sans doute racheter sa liberté à l’un de ses frères… et avec l’une de ses bagues, peut-être même les deux.>(Aislinn)

C’était encore pire que ce qu’elle avait anticipé ; dépassée, elle n’osait pas regarder Digne droit dans les yeux, tant elle regrettait de l’avoir amené dans pareil endroit.

Fort heureusement et ainsi qu’ils le découvrirent quelques minutes plus tard, si toutes les pièces de la villa étaient assaillies de thaaris, toutes ne présentaient pas un niveau d’agitation semblable à celle dans laquelle ils avaient débouché à leur arrivée. En errant un peu au hasard — et en passant notamment devant un bassin où se baignaient des Vaanis nus comme le jour de leur naissance —, ils finirent par trouver dans une annexe de la demeure beaucoup plus calme.

Soulagés, ils acceptèrent les verres de vin chaud aux épices qu’un serviteur leur proposa, avant de dénicher une banquette sur laquelle ils s’installèrent. De Varlar, il n’avait pas encore repéré l’ombre d’une trace, mais Aislinn ne se sentait pas totalement remise de sa première immersion dans l’hystérie des grands de Thaar pour l’affronter, lui et son potentiel prétendant.

[Je ne sais pas ce que l’avenir nous réserve, Digne,| souffla-t-elle à son oreille en se lovant contre lui,| mais j’espère que jamais nous ne ressemblerons à *ça*.](Aislinn)

Elle resta pelotonnée contre lui quelques secondes supplémentaires, avant de darder des prunelles soupçonneuses sur les autres âmes présentes dans la pièce. Mis à part quelques riches marchands eux aussi confortablement installés, la principale attraction de cette partie de la fête était le spectacle de deux danseuses et d’un barde.

[Au moins, ici, les gens semblent un tant soit peu civilisés.](Aislinn)
_______________________personnage_______________________
                                                       Sanaa
______________________________________________________

Sanaa n’eut guère besoin de plus qu’une petite heure pour s’en convaincre : Sauveur la fascinait.

Malgré les rires et les traits d’esprit, malgré les sourires et les tapes amicales dans le dos, ce qu’elle voyait elle, c’était cette violence contenue qui ne quittait jamais tout à fait ses prunelles, qui faisait écho à la sienne. Elle s’était rendu compte, récemment, qu’elle avait passé la moitié de son existence en tant qu’esclave ; depuis ses sept ans, elle était une babiole qui s’achetait au prix qu’on voulait bien lui accorder. Aislinn était devenue comme un sœur à qui elle confierait sa vie sans la moindre hésitation, mais Varlar n’avait pour elle que du mépris et chaque fois qu’il la menaçait de la reprendre à sa protégée et de la revendre pour une bouchée de pain, la rage de ne pas s’appartenir se rappelait à elle.

Elle aurait voulu avoir les poings du gladiateur ; elle aurait voulu pouvoir dégager cette aura invisible, mais impossible à ignorer, qui sourdrait de chacun de ses gestes et s’agitait à chacune de ses paroles.

Avant longtemps et sa troisième coupe de vin aux épices aidant, son esprit commença à vagabonder. Elle voulait le posséder et s’offrir, le goûter et se sentir désirée. Elle voulait qu’il vît en elle une femme, pas une chose. Elle le voulait ; son corps le voulait en elle.

D’abord, par pudeur, elle fit son possible pour le cacher. La présence d’Aimé la paralysait presque. Elle craigaint que l’aîné des Hadjaoui la jugeât. Pourtant et à mesure que la soirée s’étirait, des mots et des gestes lui échappaient et elle crut lire un ou deux fois une lueur d’intérêt dans le regard de Sauveur ; ces flammes discrètes dans les yeux du gladiateur lui mirent les sens en feu et inhibèrent un peu plus ses scrupules.

_______________________personnage_______________________
                                           Aislinn Noblegriffon
______________________________________________________

Aux paroles douces et rassurantes du jeune homme, elle opina lentement du chef. Elle ne savait plus à quoi elle s’était attendue, avant d’accepter de rejoindre Varlar dans cette maudite villa, mais elle comprenait mieux les manœuvres du Langecin désormais. Il aurait pu organiser la rencontre entre sa protégée et le prétendant qu’il avait choisi pour elle dans sa demeure, mais Aislinn aurait alors été dans un environnement connu et confortable, dans lequel elle aurait pu raisonner la tête froide. Malgré ses efforts, l’adolescente n’était apparemment pas parvenue à endormir tout à fait la méfiance de son tuteur, qui espérait sans doute que le bruit, l’alcool et la décadence de la soirée pousseraient la Rivoise à… Quoi ? La faute ?

[Je sais,| répondit-elle finalement à son compagnon.|](Aislinn)

Elle avait parlé si doucement que, couverte par la musique, sa voix avait dû être parfaitement inaudible pour Digne. Elle lui sourit quand il l’interrogea du regard, mais plutôt que de se répéter, préféra enchaîner.

[La prochaine fois que je t’inviterai quelque part, je choisirai un lieu plus propice à une soirée agréable,| lui promit-elle et ses yeux riaient pour elle.|](Aislinn)

Son attention fut attiré par des mouvements sur sa gauche et elle remarqua que trois couples dansaient en marge de la performance du troubadour et de ses accompagnatrices. Ses joues rosirent tandis que son inconscient lui souffla une idée.

[À moins que… nous ne sommes pas condamnés à forcément passer un mauvais moment.](Aislinn) Elle fit une légère pause, avant d’ajouter : [Sanaa m’a appris les bases de certaines danses vaanies…](Aislinn)

Elle ne se sentait de toute façon pas le courage d’affronter Varlar sur l’instant. Elle avait cru qu’il lui suffirait de se frayer un chemin jusqu’à lui, de rencontrer le Péninsulaire qu’il avait fait venir pour elle, de leur expliquer à tous deux qu’elle n’était pas à troquer et à les planter là. C’était toujours son intention que de le faire, mais elle avait besoin de se détendre un peu avant… et pas avec le verre qu’on lui avait donné quelques minutes plus tard et auxquels elle n’avait pas touché.

Elle se redressa légèrement, puis remit son châle en place, avant de plonger dans les yeux de Digne son regard à elle.
_______________________personnage_______________________
                                                      Sanaa
______________________________________________________

Aux jeux de Sauveur, Sanaa réagit de différentes manières à mesure que la soirée avançait.

Dans un premier temps, elle douta. Les initiatives du gladiateur faisaient écho à ceux qu’elle s’interdisait et elle crut qu’elle devait y voir un reflet de son propre désir qu’elle projetait bien malgré elle sur des gestes nonobstant innocents. Ensuite, elle prit peur. Lorsqu’il lui susurra sa promesse à l’oreille, elle manqua lâcher ce qu’elle tenait à ce moment-là. L’hésitation, d’un coup, n’avait plus de sens, car ses pensées n’étaient plus un fantasme qu’elle pouvait étouffer quand elle le souhaitait.

Il lui suffit cependant de croiser le regard de Sauveur pour que ses atermoiements fussent balayés, brûlés par un désir puissant qu’elle n’avait jamais ressenti jusqu’alors. C’était étrange et déroutant, un peu effrayant toujours, mais enivrant surtout. Elle avait envie d’y céder tout de suite et maintenant, sans plus attendre, sans plus mentir. Dans ce jeu de cache-cache que le Vaani avait initié, elle avait vaguement conscience d’être la souris. Elle s’en moquait ; il pouvait bien obtenir tout ce qu’il voulait d’elle, car il n’était rien qu’elle ne souhaitât pas lui donner, quand il était tant qu’elle rêvait de lui prendre.

Finalement, Sanaa se révéla, contrairement aux promesses de Digne, une bien piètre hôtesse ; à tout le moins pour Aimé, qui ne se rendit fort heureusement compte de rien. Elle observa son cadet manipuler le Hadjaoui et se *réjouit* de le voir rapidement sombrer dans la folie douce induite par la boisson. Elle rit avec lui, le moqua et l’asticota, seulement pour l’encourager à la paresse quand il tenta de se lever.

[Tu peux prendre la chambre d’Aislinn,| proposa-t-elle spontanément quand Sauveur referma finalement son piège sur sa victime.|](Sanaa)

Dans cette dernière, il y avait deux lits, mais Sanaa se souciait peu de ne pouvoir dormir cette nuit-là dans ses propres draps. Elle avait une autre idée en tête, étrange et peut-être un peu malsaine, dont elle ne savait trop si elle devait blâmer l’alcool ou sa trop grande rancœur à l’égard de l’homme qui jamais ne lui reconnaîtrait son droit à sa liberté.

Elle se donnerait à Sauveur dans le lit de Varlar.

Elle tacherait les draps du vieux Langecin avec ses premiers sangs.

Ce serait son ultime acte de défi, sa plus belle marque de mépris et elle se moquait d’ignorer s’il découcherait ou non ce soir-là ; elle subirait son courroux sans broncher si elle le devait.

Et tandis qu’elle guidait Sauveur, qui lui-même soutenait Aimé, jusque dans leur sanctuaire à elle et Aislinn, elle ne pouvait plus penser à rien d’autre qu’à son cœur qui battait la chamade et son bas-ventre qui la chauffait délicieusement.

_______________________personnage_______________________
                                            Aislinn Noblegriffon
______________________________________________________

La chaleur des doigts de Digne sur son dos arracha plus d’un frisson à la Rivoise. Sous les caresses du jeune homme et les subtils picotements que chacun de ses mouvements provoquait de la base de sa nuque au creux de ses reins, elle sentait que son épiderme *s’éveillait*. Avant longtemps, elle dardait sur lui un étrange regard ; pour mille et une raisons, il lui était difficile d’interpréter les sensations qui l’envahissaient en cet instant présent, mais elle était certaine d’une chose : dans les bras de Digne et tandis qu’il dessinait avec tendresse la courbure de son dos, elle lui vouait une confiance absolue. Elle savait qu’elle n’avait rien à craindre ; ni ses gestes à lui, ni ses sentiments et ses émois à elle.

S’il put croire avoir été trop loin et faire mine de retirer ses mains, elle l’en empêcha en saisissant ses coudes et en secouant lentement — intensément — la tête.

[Je t’aime, Digne Hadjaoui,| lui avoua-t-elle.|](Aislinn)

Dans cette période charnière de son existence, tandis que l’incertitude sur l’avenir s’ajoutait aux doutes sur la vertu des projets qu’elle commençait tout juste à fomenter dans le secret de son esprit, elle comprenait enfin qu’*il* était cette ancre à laquelle elle pouvait se raccrocher, l’îlot paisible sur lequel elle pouvait se réfugier, le bras sur lequel elle pouvait compter pour l’aider à se relever.

Et la Rivoise de chercher à cueillir ses lèvres, une nouvelle fois.
________________________personnage_____________________
                                                        Sanaa
______________________________________________________

Tout allait très vite.

Sanaa était complètement dépassée par la fougue de Sauveur ; elle ne s’en plaignait pas, c’était même tout le contraire, mais elle avait du mal à savoir quoi faire, comment se comporter ou quand agir. C’était donc le gladiateur qui, pour l’heure, prenait la plupart des initiatives et elle espérait juste qu’il ne prît pas ombrage de son manque d’audace à elle.

Son cœur battait si vite.

Tandis qu’il explorait son cou avec ses lèvres et sa langue, elle emmêla ses doigts dans ses cheveux et quand elle sentit ses mains tirer sur sa robe, une puissante décharge d’adrénaline fouetta son sang. Elle était un magma bouillonnant de tant et tant d’émotions qu’elle n’essayait même plus d’en faire le tri.

[Ne restons pas là,| dit le jeune homme et sa voix agit comme la plus érotique des caresses.|](Sauveur)

Elle darda sur lui des prunelles brûlantes, muette et agitée du seul mouvement de sa poitrine qui se soulevait et s’abaissait au rythme de son souffle court.

[Viens,| finit-elle par lui intimer en mêlant ses doigts aux siens ; sa voix était rendue légèrement rauque par le désir qui lui serrait la gorge et chauffait le bas-ventre.|](Sanaa)

Son plus hésiter le moins du monde, elle le conduit comme elle l’avait prévu dans la chambre de Varlar, qui était à l’image du personnage et de sa demeure. Guidant toujours Sauveur, elle le plaça dos au matelas, puis l’y poussa sans douceur pour qu’il s’y allongeât. Elle défit une à une les attaches de sa robe, puis laissa glisser les étoffes sur sa peau hâlée, le regard brillant de ce désir qu’elle ne comprenait pas totalement, mais ne craignait pas ni ne cherchait à réprimer. Elle se débarrassa des dernières pièces de tissu qui la couvrait encore, avant de le rejoindre.

Elle se pencha sur lui, un bras de chaque côté de son corps, puis l’embrassa avec une fougue alimentée par sa propre nudité ; elle se redressa ensuite juste assez pour pouvoir l’aider à retirer sa tunique.

Enfin, elle alla mordiller son oreille droite, pour lui murmurer : [Prends-moi. Maintenant.](Sanaa)

Elle avait entendu sa première maîtresse dire cela à son amant, une fois ; il lui avait semblé que le concerné y avait été plutôt réceptif…

________________________personnage_____________________
                                                        Varlar
______________________________________________________

Varlar était beaucoup de choses, mais un exubérant ça, jamais. Il suffisait de visiter sa demeure une seule fois pour s’en convaincre. Le vieux Langecin se satisfaisait bien assez de vivre *bien*, ce qui voulait dire dans sa bouche à l’écart du besoin. Pour le reste, il mesurait son succès à l’or qu’il parvenait à amasser, non aux babioles qu’ils pouvaient se procurer en le dépensant ; l’expérience lui avait par ailleurs appris que la monnaie restait bien plus efficace pour solder des dettes. À Thaar, cette manière d’être ne vous menait jamais très loin ; les marchands des principautés se moquaient bien de vos coffres remplis d’espèces sonnantes et trébuchantes, s’ils ne pouvaient pas les jauger du premier regard en vous détailler de pied en cape.

Même à Ys, où il avait surtout commercé avec des petits bourgeois sans beaucoup d’envergure, le Langecin avait dû sacrifier aux coutumes locales ; depuis qu’il était à Thaar, les simagrées auxquelles il devait se livrer atteignaient des proportions inimaginables. Il avait par exemple était contraint d’investir dans une large garde-robe quand il avait compris que le voir vêtu de son éternel pourpoint sombre — fût-il toujours impeccablement lavé — dégoûtait ses partenaires.

Pour l’heure, il se pliait à ces règles et ces coutumes absurdes parce qu’elles le servaient bien ; cependant, ce n’était pas un hasard si, quelques mois plus tôt, son regard s’était tourné à nouveau vers la Péninsule. Quand la Noblegriffon lui avait refourgué sa prétendue progéniture, il n’avait pas prévu de s’occuper d’elle au-delà du strict minimum… et son bonheur matrimonial était bien au-delà de ses considérations initiales. Le mal du pays l’ayant finalement rattrapé, il avait décidé de s’enrichir le plus et le plus rapidement possible, de sécuriser ses gains, puis de quitter ces fous sans l’ombre d’un regret.

[Avec les grands jeux, les mires de nos amis estréventins seront rivées sur leur foutue arène,| expliquait-il à la personne à côté de lui dans sa langue maternelle.| C’est le moment de faire des affaires.](Varlar)

Il avait bien pris soin d’expliquer à son interlocuteur, il fallait toujours partir du principe que les thaaris parlaient tous tous les dialectes. La plus grande ville du monde n’avait pas usurpé son surnom et on ne concentrait pas une telle population simplement en y semant sa progéniture.

[Bien sûr, une fois que vous aurez épousé Aislinn…| voulut-il continuer avant d’être coupé.|](Varlar)

[*Si* j’épouse votre pupille,| précisa le Péninsulaire et son accent trahissait ses origines Nordiennes.|](Eloi)

[Évidemment,| en convint le Langecin en dardant sur Eloi du Vertbois un regard aussi placide que faire se pût.|](Varlar)

Le chevalier, qui venait de Serramire, était jeune, tout du moins du point de vue de Varlar ; cela ne l’empêchait pas, du haut de sa trentaine, d’être déjà veuf de sa première épouse, morte en couche trois années plus tôt. Le Langecin avait longtemps hésité, avant de décider où jeter ses filets ; il avait finalement choisi le duché des marches du Royaume plutôt que Langehack dans l’espoir de se rapprocher ainsi des quelques « comptoirs Noblegriffon » qui subsistaient toujours dans le Nord. Si on lui avait posé la question, quelques jours après la disparition de Katalina, jamais Varlar n’aurait désigné la terre natale de la Noblegriffon pour fomenter son retour, car Aislinn n’avait pas les traits de sa prétendue mère et moins encore ceux d’une Nordienne. Cependant, le Langecin avait en sa possession des documents qui attestaient que la gardienne l’avait reconnue comme telle et depuis que ses agissements en Barkios des Ans 11 et 12, il avait le sentiment que personne ne prendrait le risque de l’accuser de parjure.

[Elle va avoir seize ans, vous dites,| continua Eloi sans se départir de son air austère.|](Eloi)

[Pas tout à fait, mais ça ne saurait tarder.](Varlar)

[Elle a déjà vécu sa première floraison ?| demanda-t-il encore sans sourciller.|](Eloi)

Varlar n’en avait aucune idée ; il employait une femme de chambre presque exclusivement pour ne pas avoir à se soucier de ce genre de questions. Il allait improviser une réponse qui, il l’espérait, embobinerait son indélicat interlocuteur juste ce qu’il fallait pour lui permettre de changer de sujet, mais ce dernier se figea avant de darder sur son aîné un regard indéchiffrable.

[Elle est encore intacte, bien entendu ?](Varlar)

Cette fois, le Langecin s’empourpra tout à fait, moins pour s’offusquer du peu de cas qu’Eloi faisait de l’honneur de sa protégée que pour ce que cela sous-entendait de l’éducation qu’il lui prodiguait, lui.

Qu’elle fut pratiquement inexistante, en tout cas sur ces aspects en particulier, était hors de propos.

[Ce n’est pas la peine de me regarder avec ces yeux-là, Varlar,| s’agaça le Nordien avant de tendre le bras pour désigner quelque chose dans son dos,| car sauf erreur de ma part, votre protégée est arrivée… et elle semble profiter de sa soirée, contrairement à nous.](Eloi)

Et le Langecin de manquer faire une crise d’apoplexie quand, se retournant, il découvrit la timide — et un peu gourde, de son point de vue — Aislinn en train de bécoter un Vaani par trop jeune et séduisant à son goût.

[Par le con de Tyra,| s’étrangla-t-il avant de planter là le Vertbois qui lui emboîta tout naturellement le pas.| Aislinn ! Veux-tu bien t’éloigner de ce…](Varlar)

Il retint l’insulte qui lui chatouillait la gorge de justesse ; pour ce qu’il en savait, ce jeune homme pouvait très bien être l’amant d’un prince ou l’héritière d’une princesse. Avec leur sexualité débridée et leur mauvaise manie de tous être plus riches les unes que les autres, il devenait dangereux de simplement se fier aux apparences. Or, d’apparence, le cavalier aventureux de la Rivoise ne payait pas forcément de mine.

[Varlar,| l’accueillit froidement Aislinn après avoir rapidement remis un peu d’ordre dans sa tenue.|](Aislinn)

Non seulement la garce restait aux côtés de son gigolo, mais en plus elle le défiait ; il pouvait voir dans son regard une détermination qu’il ne lui connaissait pas.

[Sire,| enchaîna-t-elle sans laisser à son tuteur le loisir de lui répondre.| Vous devez être l’homme qui souhaite m’épouser.](Aislinn) Elle se fendit d’une révérence tout à fait appropriée — contrairement à son attitude et à sa tenue —, avant de tourner son visage vers son partenaire de danse. [Je vous présente Digne Hadjaoui.](Aislinn)

[Et qui est-il pour vous, ma dame ?| demanda Eloi sans se départir de son calme.| Si vous m’autorisez l’audace de vous poser la question.](Eloi)

Sa mâchoire carrée était crispée et ses prunelles glaciales, mais au moins savait-il se comporter en société ; il était d’une politesse irréprochable.

[Sire,| répondit l’insolente en coulant subrepticement un regard au concerné avant d’esquisser un sourire,| il est bien des choses en vérité ; un ami, un confident… et le compagnon pour qui mon cœur bat chaque jour un peu plus.](Aislinn)

C’en était trop pour Varlar, qui — et c’était là un événement rare — perdit patience ; la gourgandine était d’une impudence crasse et son attitude était une bien vile manière de le remercier de tout le mal qu’il s’était donné pour elle. <Telle mère, telle fille|, songea-t-elle tandis qu’une bouffée de haine lui serrait la gorge.| Des ingrates, toutes les deux.>(Varlar) Il était *hors de question* que cette petite crise adolescente ruinât tous ses efforts.

Il devait montrer à Eloi qu’elle lui obéissait.

Il devait effacer cet affront.

Aislinn s’était naturellement placée entre lui et Digne, mais ce n’était pas sur le fameux Hadjaoui qu’il portait son attention. Il leva la main, avec la ferme intention de la gifler. Le Vertbois s’en rendit compte un battement de cœur trop tard et ne put retenir son poignet.
______________________personnage________________________
                                                    Sanaa
______________________________________________________

Quelque chose n’allait pas.

Tandis que Sauveur s’occupait d’elle et que des vagues successives de plaisir lui faisaient perdre pied, Sanaa ne pouvait se départir d’une certaine frustration qu’elle ne comprenait pas. Pourtant, ses ébats avec le gladiateur auraient dû la combler ; l’Hadjaoui s’était vite révélait être un amant attentionné et passionné, expert dans l’art de provoquer et d’entretenir l’excitation d’une femme et Sanaa n’avait jamais rien ressenti de comparable auparavant.

Quand il embrassa à pleine bouche son intimité, électrisant d’un seul coup tout son corps et sa conscience, tout devint limpide.

En creux de son désir pour Sauveur, dès le départ, il y avait cette envie de transgression qu’elle portait sur elle comme un manteau depuis des années. Elle avait souhaité se donner à son amant du soir autant pour le plaisir charnel qu’elle en retirait que pour s’extirper de cette laisse invisible que Varlar continuait d’utiliser contre elle ; le choix du lieu de leurs ébats ne laissait rien au hasard. Or, depuis qu’elle avait fait choir sa robe sur le sol, Sanaa ne maîtrisait plus rien. Volontairement ou non, le gladiateur avait soigneusement gardé le contrôle des événements, en imposant son rythme et en se refusant à satisfaire la demande pressante de la Vaanie.

Et elle avait aimé cela.

Parce qu’elle était jeune et inexpérimentée, impressionnée par son partenaire contre elle, curieuse des plaisirs qu’il l’aidait sans le savoir à découvrir ; pour ces raisons et mille et mille autres, elle s’était laissé guider et son acte de transgression, finalement, était devenu une initiation et, mécaniquement, une nouvelle forme de subordination.

D’où sa frustration.

Sanaa ne voulait pas être une simple spectatrice de leurs ébats ; elle désirait s’émanciper, participer, imprimer sa marque… et dans le même temps, chaque geste, chaque caresse, chaque coup de langue de son amant la faisait chavirer un peu plus.

Elle voulait qu’il arrêtât ce qu’il était en train de faire et qu’il la prît sans plus tarder.

Elle voulait qu’il continuât son jeu du chat et de la souris avec son clitoris.

Ses mains, qu’elle avait gardées autour de sa tête à elle depuis qu’il avait commencé sa — trop — lente descente jusqu’à son bassin et en dessous, entreprirent un voyage similaire le long de son corps. Elle se caressa furtivement les seins, découvrant cette étrange sensation de se donner un peu de plaisir quand un mâle s’activait à faire de même, mais ses doigts ne s’attardèrent guère sur ses mamelons et quelques instants plus tard, elle glissait ses phalanges dans la chevelure du gladiateur. Elle commença à y imprimer une légère pression pour l’encourager à continuer, tandis qu’elle s’arquait pour lui faciliter l’accès à son intimité.

Peu de temps avant que Sauveur ne parvînt totalement à ses fins en déclenchant en elle un premier orgasme, elle croisa ses jambes en tailleur sur son dos et se cambra plus encore… pour, au moment fatidique et surprise par l’intensité de ce qu’elle ressentait, faire le mouvement inverse et enserrer la tête du jeune homme contre ses cuisses.

Il était bon que l’alcool aidât Aimé à dormir, car Sanaa ne fit aucun effort pour réprimer ses gémissements.

______________________personnage________________________
                                          Aislinn Noblegriffon
______________________________________________________

Il était ainsi qu’elle l’avait imaginé. Trop vieux, trop austère, trop sérieux.

À l’image du traître qui l’avait attiré jusqu’ici.

Aislinn s’était souvent demandé ce qu’elle devait penser des agissements de Varlar. Il avait été honnête avec elle et n’avait rien caché de ses projets. Il *était* son tuteur légal, chargé par sa mère de prendre soin d’elle et de préparer son avenir. D’un point de vue froidement «stratégique», retourner en Péninsule dans le duché natal de la gardienne faisait sens, si l’on acceptait l’idée qu’après près de quinze longues années d’absence, le nom de Noblegriffon avait une quelconque légitimité à présider la destinée des comptoirs de Katalina avait fondé au crépuscule de Xe cycle. Il ne lui avait certes jamais demandé ce qu’elle pensait de tout cela, mais dans le même temps, elle avait laissé filer bien des occasions de faire valoir son opinion. <Peut-être Sanaa avait-elle tort et j’ai commis une grave erreur|, songea-t-elle non sans anxiété tandis que le Langecin furibond s’approchait.| Il m’aurait peut-être écouté.>(Aislinn)

La suite devait balayer ses doutes, car mis en défaut, Varlar perdit complètement pied, jusqu’à chercher à purement et simplement la frapper.

Elle le vit lever le bras, main tendue, et se surprit à se préparer instinctivement au choc plutôt que d’essayer de *faire quelque chose* pour se soustraire à son coup. Digne, quant à lui, se montra plus réactif et parvint à arrêter la paume du vieil homme avant qu’elle ne pût s’abattre sur le visage de l’héritière.

<À quoi ont servi toutes ces heures d’entraînement, si le moment venu, je reste la même petite fille sans défense que j’ai toujours été ?| se demanda-t-elle, choquée malgré elle par la violence à laquelle elle avait échappé.|>(Aislinn)

Elle n’entendit pas les paroles dures du benjamin des Hadjaoui, car son regard avait accroché celui d’Eloi. Le chevalier ne cachait pas son agacement, mais elle ne parvenait pas à déterminer quel élément de sa désastreuse soirée l’énervait le plus. Il avait essayé de s’interposer et elle lui était indubitablement reconnaissante de ce simple geste, eût-il été vain. À sa grande surprise, il inclina légèrement la tête dans une forme d’amende honorable.

[Hadjaoui parle vrai, Varlar,| intervint le Serramirois dans la langue diantraise sans se départir de son ton mesuré,| quoique, ma propre fille n’ayant pas survécu, je serai bien en peine de vous donner le moindre conseil en la matière.](Eloi)

[Les amourettes ridicules d’une adolescente ne devraient pas remettre en cause les projets dont nous avons déjà largement discuté,| protesta le Langecin sans plus chercher à feindre le calme.|](Varlar)

[Elles changent tout,| le contredit Eloi en posant cette fois son regard sur Digne.| Je n’ai guère le goût de forcer une femme à partager ma couche et je gage que votre protégée se battra pour s’en tenir éloignée tant qu’il lui restera une once d’énergie.](Eloi) Il reporta son attention sur Varlar. [Ne faites pas l’enfant, vous êtes trop âgé pour cela. Le mariage n’est absolument pas nécessaire à nos arrangements.](Eloi)

Assurément, le concerné ne s’était pas attendu à ce que la soirée se déroulât de la sorte, mais il lui semblait que tout et tous se liguaient contre lui. Isolé, il resta coi quelques secondes.

[Si cela peut atténuer votre rancœur, Varlar, je ne comptais pas épouser Aislinn. J’ai besoin d’or et c’est ce que je suis venu chercher,| finit par intervenir une nouvelle fois Eloi.| Je demeure un vassal et je doute que mon suzerain soit ravi à l’idée d’apprendre que j’ambitionne de devenir le gendre de la gardienne de Tyra qui a tué un archonte et un roi en une petite année.](Eloi) Il darda un regard légèrement amusé pour Aislinn, avant d’ajouter : [En vérité, je ne pense pas que vous parveniez un jour à la marier de mon côté de l’Olienne.](Eloi)

[Ce ne serait pas la première fois que Varlar agit hâtivement sans avoir pris le temps de considérer tous les tenants et aboutissants,| fit remarquer froidement une Aislinn qui était restée étrangement silencieuse jusque là.|](Aislinn)

Elle chercha ensuite le visage de Digne des yeux, curieuse de découvrir comment il réagissait à l’annonce du chevalier. Elle-même était encore trop secouée pour vraiment réaliser ce que ses paroles impliquaient.
______________________personnage________________________
                                                    Sanaa
______________________________________________________


De doutes de Sauveur, de sa fulgurance et de la colère qu’elle provoqua, Sanaa ne devina rien. Elle était trop portée par les sensations vivaces qui continuaient d’affluer de chaque parcelle de son épiderme et des tréfonds de son bas-ventre pour s’en soucier. Elle ne prit donc aucunement la mesure de la prévenance du gladiateur, quand il la plaça au-dessus de lui pour la laisser maîtresse de ce qui allait suivre. Le souffle court, le regard brûlant, elle le dévora des yeux tandis qu’elle ondulait lentement son bassin et qu’elle lui frayait un chemin en elle.

Toute sa frustration passée s’était complètement évaporée.

Elle agrippa les épaules de son amant, se cambra et rejeta la tête en arrière au moment où elle achevait sa descente ; il y avait quelque chose de libérateur dans la douleur qui l’envahit lorsque son hymen se déchirait.

Le gladiateur ne resta pas inactif et elle le laissa faire, encore déboussolée par ce qu’elle était en train de ressentir. Il commença à bouger en elle et la douleur revint, lancinante toujours, mais moins forte aussi, à la manière d’un écho que chaque rebond atténuait ; et au diapason de cette souffrance qui peu à peu s’effaçait, il était une seconde vague qui montait. C’était si étrange que l’une ne parvînt pas à étouffer l’autre ; quand la respiration de Sauveur s’accéléra et qu’elle le sentit chercher à ralentir ses va-et-vient, elle le prit à contre-pied, accentuant ses propres mouvements de bassin, elle le porta jusqu’à l’orgasme et ses gémissements à elle complétèrent subtilement ses râles à lui.

Ils se figèrent tous les deux, essoufflés et couverts de sueur, puis Sanaa se dégagea précautionneusement de lui, prenant bien soin de ménager son intimité endolorie malgré la prévenance de son amant. Sans un mot, mais avec un sourire ravi, elle se pencha ensuite sur lui et lui vola un dernier baiser passionné, avant de s’allonger à ses côtés et de commencer à lui caresser le torse.

_______________________personnage_______________________
                                                      Sanaa
______________________________________________________

<Quand ?| avait-elle envie de lui répondre.| Quand aurais-je dû te le dire ?>(Sanaa) Elle avait beau essayé de faire preuve d’imagination, elle ne voyait pas à quel moment elle aurait pu prévenir son amant de sa virginité ; pas sans risque de gâcher ce que son ventre avait appelé de ses vœux dès l’instant où elle avait laissé pour la première fois glisser son regard sur sa personne.

Ce qu’ils venaient de faire ensemble, elle ne l’avait ni anticipé ni préparé ; elle s’était contentée de le *vivre*, complètement dépassée par le charme presque magnétique de Sauveur. Il ne pouvait pas lui reprocher quoique ce fût ; pas après l’avoir séduite comme il l’avait fait. Elle avait très vite compris qu’elle le désirait, mais avant qu’il commençât à jouer avec elle dans la cuisine, elle n’aurait jamais osé passer à l’acte ; une fois qu’il lui avait murmuré son invitation détournée dans le creux de son oreille, tout avait changé. Elle darda sur lui son regard clair, mais se garda bien de lui répondre ; ce n’était pas tant qu’elle ne le voulait pas, c’était surtout qu’elle ne savait pas *quoi* lui dire.

Malheureusement, Sauveur prit son silence pour ce qu’il n’était pas.

[Je parle du fait que tu étais vierge,| crut-il judicieux de préciser.|](Sauveur)

Elle aurait préféré qu’il la giflât ; c’était irrationnel et elle s’en rendit compte, mais elle rejeta cet éclat de sagesse et embrassa les caprices du Prisonnier.

[J’avais deviné,| grinça-t-elle en rassemblant ses jambes sous elle avant de s’agenouiller, nue comme le jour de sa naissance.|](Sanaa)

Quelques secondes plus tôt, elle reposait contre lui, contentée et heureuse ; elle dardait désormais sur lui des prunelles courroucées sans même chercher à comprendre *pourquoi*. <Ne pouvais-tu pas te satisfaire de ce que nous avions ?| avait-elle envie de lui demander.|>(Sanaa) Elle avait la désagréable impression que le gladiateur était déçu, finalement, du temps qu’il avait passé avec elle ; elle n’aurait pas cru que cette pensée la mortifierait autant.

[Je ne vois pas pourquoi j’aurais dû,| finit-elle par asséner et elle n’avait besoin de personne pour réaliser qu’elle faisait preuve d’une mauvaise foi évidente ; cela l’agaçait encore plus.|](Sanaa)

Elle pouvait l’entendre lui expliquer ce qu’elle avait déjà compris. Elle avait beau être jeune et ignorante de bien des arcanes de l’amour, elle *savait* que Sauveur avait été particulièrement patient et prévenant à son égard ; l’idée qu’il se servît de cela pour la sermonner accentua un peu plus sa contrariété.

Elle voulait juste qu’il se tût et qu’il la laissât se blottir à nouveau contre lui.

[J’en avais envie. Tu en avais envie. Les choses n’avaient pas besoin d’être plus compliquées que cela.](Sanaa)

______personnage_________
Aislinn Noblegriffon
________________________

[Je cherche quelqu’un,| répondit Aislinn sans parvenir à détacher son regard de la crinière rousse de l’aubergiste.|](Aislinn) <Les mêmes qu’Aimé, j’en suis certaine|, se répétait-elle silencieusement sans réussir à tirer quoique ce fût de cette constatation.|>(Aislinn) Assurément, son impromptu interlocuteur n’était pas l’homme qu’elle recherchait. Il était beaucoup plus jeune que l’ancien gladiateur qui, deux ans auparavant seulement, l’avait juchée sur son épaule. Sans s’en rendre compte, elle le détaillait des pieds à la tête, indécise sur la marche à suivre ; il fallut l’intervention de Sanaa pour la ramener à la réalité. Son amie se racla ostensiblement la gorge en se postant à ses côtés, dardant des prunelles noires et amusées sur le marteau rescapé.

[Quelqu’un qui, si j’en crois les --- nombreuses --- descriptions d’Ellie, vous ressemblent beaucoup,| expliqua-t-elle avec une malice évidente.|](Sanaa)

[Aimé Hadjaoui,| la coupa Aislinn en se sentant rougir sans être vraiment certaine de pourquoi.| Je cherche Aimé Hadjaoui. Lorsque nous nous sommes rencontrés, il m’avait dit que si jamais je devais voyager à Thaar, je ne devais pas hésiter à passer le voir.](Aislinn) Elle se garda bien de préciser que son invitation remontait à deux années pleines, pas plus qu’elle ne mentionna son périple pour parvenir jusqu’ici. Elle coula un regard en biais à Sanaa, qui lui confirma d’un sourire qu’elle resterait muette à ces sujets elle aussi. <Elle me connaît trop bien|, constata une énième fois l’héritière Noblegriffon.|>(Aislinn) [Est-ce qu’il travaille toujours ici ?| l’interrogea-t-elle en essayant de retrouver un semblant de contenance.|](Aislinn)

Elle marqua une légère pause, puis se rendit compte qu’elle ne s’était même pas présentée et se hâta de corriger son erreur, coupant potentiellement la parole au jeune homme tandis qu’il commençait à lui répondre. [Je m’appelle Aislinn et voici Sanaa,| annonça-t-elle en désignant la Vaanie d’un geste de la main.|](Aislinn)

[Enchantée,| la salua l’adolescente en se fendant d’une courbette enjouée.|](Sanaa)
________________________personnage______________________
                                                         Sanaa
______________________________________________________

Dès l’instant où le gladiateur avait décidé de soulever le sujet de sa virginité, Sanaa s’était demandé où Sauveur voulait en venir ; il ne comprenait pas ce qu’il espérait obtenir d’elle à la confronter ainsi, une fois leurs ébats terminés. Dans quelques heures tout au plus, ils étaient appelés à se séparer et la Vaanie doutait fortement qu’ils eussent l’occasion de se recroiser avant longtemps, moins encore de partager à nouveau une couche.

Quand bien même elle avait effectivement commis une erreur en ne le prévenant pas que son hymen était intact au début de leurs étreintes, cette affaire-là appartenait au *passé* et il était peu probable que la membrane qui gardait jusqu’à récemment l’entrée de sa matrice se reformât un jour ; Aislinn lui avait bien expliqué que là-bas en Péninsule, pendant la Malenuit, certaines femmes trop proches de l’Ailée étaient redevenues vierges, mais enfin Sanaa ne pariait pas être témoin d’un nouveau Voile de son vivant.

Sauveur souhaitait comprendre les raisons de son mutisme ? Elle était de son côté indubitablement curieuse de deviner *pourquoi* ; autrement dit et pour un temps au moins, la conversation entre les deux amants avait peu de chance de finir sous les meilleurs auspices.

[Tu veux que je te remercie, en fait ?| demanda Sanaa avec un ton qui laissait planer le doute sur ce qu’elle pensait de cette idée.|](Sanaa)

Ce n’était pas exactement ce qu’il avait dit, mais c’était l’essentiel de ce qu’elle avait retenu. Bien entendu, dans le fond, il avait *raison* : s’il avait pris moins garde à ménager son corps jusqu’alors préservé, les chances étaient grandes que leurs ébats eussent duré beaucoup moins longtemps ; quoiqu’ils ne pouvaient pas le savoir, car de son point de vue, elle n’avait pas eu *si* mal que ça quand il avait essayé de la pénétrer la première fois.

[Dans ce cas, merci, Sauveur,| déclara-t-elle sans changer d’intonation.| Grâce à toi, mon premier coït restera un souvenir impérissable.](Sanaa)

Il lui suffit de prononcer ces mots pour se persuader qu’ils n’étaient pas dénués de vérité. Elle laissa ensuite son regard se promener sur le corps de Sauveur pour la première fois depuis qu’ils avaient commencé leur houleuse discussion ; cela lui permit, à tout le moins, de rafraîchir dans sa mémoire *pourquoi* ils s’étaient retrouvés nus l’un contre l’autre en premier lieu. Les lippes de la diablesse se tordirent en un sourire carnassier, puis elle s’étendit de tout son long à côté de lui, lui offrant au passage une vue de choix sur ses courbes sans le moindre regret.

Le message avait le mérite d’être clair ; de son point de vue, la conversation était close. Quant à ce qui pouvait venir après… elle-même n’en avait aucune idée.

_______________________personnage_______________________
                                           Aislinn Nobglegriffon
______________________________________________________

_________________________date__________________________
Oglicos 51 Verimios de l’An `13:XI`, le lendemain de sa rencontre avec Eloi du Vertbois, dans la demeure de Varlar.
______________________________________________________

Depuis qu’elle s’était réveillé une petite heure plus tôt, Aislinn n’avait pas esquissé le moindre mouvement. Elle s’était contentée de ressasser les événements de la veille, les yeux rivés sur le plafond.

Aislinn avait vu Varlar prendre la mesure de sa défaite, lorsqu’Eloi avait finalement révélé qu’un mariage avec l’héritière Noblegriffon n’avait jamais fait parti de ses projets. Elle avait vu le visage du vieux Langecin se fermer comme une huître et son regard dur et froid passer de sa protégée à Digne. <Il ne me fera plus jamais confiance|, avait-elle compris et elle ne pouvait pas étouffer complètement le sentiment de culpabilité que cette pensée avait engendré.| Il a toujours servi ses intérêts, mais jamais au détriment des miens.>(Aislinn) D’où ils venaient tous deux, qu’une jeune femme s’offrît en mariage à un bon parti pour s’assurer une vie confortable était la norme, pas l’exception.

La trahison était réciproque, cependant ; elle était prête à assumer ses fautes, mais aucune ne méritait qu’il levât la main sur elle.

[Maintenant que je vous ai rencontré, je ne peux m’empêcher de regretter les actes de votre mère…| avait soupiré Eloi avant d’emboîter le pas à Varlar|, si elle n’avait pas tué un Roi, je n’aurais pas hésité longtemps à faire de vous mon épouse.](Eloi)

Et le chevalier de darder un regard sur Digne, comme pour jauger quel rival il aurait fait ; le sourire en coin dont il les avait gratifiés avant de prendre congé fut l’unique indice qu’il consentit à leur donner quant au résultat de ses réflexions.

Elle avait eu de la chance.

Elle avait cru qu’il lui suffisait de tenir tête à Varlar pour mettre en défaut ses projets ; que sa volonté seule pouvait défaire cette voie qu’il avait commencé à tracer pour elle — pour lui — sans la consulter avant. Les quelques mots d’adieu d’Eloi lui avaient cruellement remémoré où était sa place ; même les regards doux et les paroles d’amour de Digne n’avaient pu totalement lui changer les idées.

[Sanaa ?| appela-t-elle son amie qui, supposait-elle, dormait toujours à quelques mètres d’elle.|](Aislinn)

Elle était rentrée tard et n’avait pas été très surprise de découvrir la demeure de Varlar entièrement vide ; il n’était pas rare que le vieux Langecin s’absentât plusieurs jours et elle ne savait pas quand il referait surface cette fois-ci — encore moins dans quel état d’esprit. Quant à Aimé et Sauveur, ils avaient dû tirer leur révérence plus tôt dans la soirée ; en tout cas Sanaa était couchée, ce qui était déjà plus inattendu. Aislinn s’était tout de même réjoui de ne pas avoir à revivre sa confrontation avec son tuteur pour la raconter à la Vaanie. Elle s’était contentée de se changer aussi rapidement et silencieusement que possible, avant de se glisser sous ses draps à son tour.

Se redressant sur son coude, l’héritière Noblegriffon fronça les sourcils quand elle posa enfin son regard sur le matelas à côté du sien. Soit Sanaa avait doublé de volume en quelques heures, soit…

[Aimé ?| lâcha-t-elle estomaquée en reconnaissant l’aîné des Hadjaoui.| Qu’est-ce que tu fais là ? Où est Sanaa ? Et…](Aislinn)

Le souvenir des événements de la veille et la manière dont elle s’était dévêtue sans gêne à ses côtés la rattrapèrent et elle manqua s’étrangler en ce sentant rougir. Il devait dormir à ce moment-là et même s’il n’en était rien, il faisait si noir qu’il n’avait rien pu voir.

[Sanaa ?| appela Aislinn en haussant la voix avec fébrilité.|](Aislinn)

La Vaanie ne mit guère de temps à les rejoindre, fraîche comme un gardon et visiblement amusée par l’étrange scène matinale qui se déroulait sous ses yeux pétillants de malice. Elle n’eut aucun mal à expliquer le fin mot de l’histoire, tandis qu’Aimé émergeait péniblement de sa soirée arrosée. Trop prompt à vider ses verres, l’ancien gladiateur avait vite sombré dans un profond sommeil ; Sauveur et Sanaa, avinés eux aussi du propre aveu de la concernée, l’avait aidé à aller se coucher dans la chambre des adolescentes… seulement pour se rendre compte qu’il ronflait si fort que jamais la Vaanie ne pourrait dormir dans son lit. Elle avait donc décidé « d’emprunter » celui de Varlar, tandis que Sauveur s’en était retourné à l’Aile Blanche.

[La nuit a été bonne, au moins, Aimé ?| l’alpagua une Sanaa résolument taquine.|](Sanaa)

Aimé gronda gentiment alors qu'il peinait à ouvrir les yeux tant la faible lumière qui éclairait la pièce lui faisait mal au crâne. Ses paupières ne semblant pas suffire à l'en protéger, il mit une main par dessus. S'il avait bien dormi ? Elle n'avait jamais dû être saoule pour poser cette question.

[Au lieu de te moquer, tu n'aurais pas plutôt un verre d'eau ? Peut-être même un pichet…](Aimé)

_________________________date__________________________
Plusieurs heures plus tard, dans le courant de l’après-midi.
______________________________________________________

Sanaa avait insisté pour qu’Aaimé déjeunât avec elles avant de s’en aller et comme souvent avait obtenu gain de cause. Le Hadjaoui ne s’était cependant pas énormément attardé, car il était attendu à l’auberge. Dès qu’elles avaient été seules, Sanaa avait assailli son amie de questions, mais Aislinn avait fermement refusé d’y répondre et s’était à la place enfermée dans le bureau de Varlar.

L’idée lui était venue en se réveillant : elle voulait profiter de l’absence de son tuteur pour tenter de percer tous les secrets qu’il gardait toujours par-devers lui. Elle ne pouvait plus se permettre de le laisser mener *leurs* affaires avec autant de liberté. Elle avait déjà commencé à enquêter dans la plus grande discrétion — elle n’avait confié ses édifiantes trouvailles à personne, pas même à Sanaa ou Digne —, mais il lui manquait encore des plusieurs pièces du puzzle. C’était sa chance de lever plus d’un voile sur les activités plus ou moins licites de Varlar et elle entendait bien la saisir. Le moins que l’on pouvait dire, c’était qu’elle n’était pas déçue. Cela faisait quatre heures qu’elle s’était enfermée dans l’antre austère de Varlar et le sol était jonché de parchemins qu’elle avait dévorés avidement avant de les abandonner sur place.

Les Cinq en fussent témoins, elle avait sous-estimé le Langecin ; le faquin trempait dans pas moins de trois opérations hautement risquées — qui pouvaient toutes leur faire perdre une bonne partie de leur capital si les calculs d’Aislinn étaient corrects — dont une qui s’appuyaient lourdement sur des réseaux et des mécaniques parfaitement scandaleuses.

Si Aislinn s’en était douté, elle en avait la preuve désormais : son or était sale.

Cela l’horrifiait, mais dans le même temps, elle savait qu’elle avait trouvé là toutes les informations qui lui manquaient pour s’émanciper de Varlar sans plus tarder.

Un cri l’arracha à ses lectures et chassa au moins momentanément de son esprit ces préoccupations qui la dépassaient. Elle avait tout de suite reconnu la voix.

[Sanaa ?| appela-t-elle en se redressant.|](Aislinn)

Elle rassembla ses robes dans ses mains pour qu’elles ne la gênassent pas, puis se mit à trottiner vers la sortie du bureau… seulement pour entendre un second hurlement, accompagné d’un juron chargé d’une colère noire.

[Espèce de petite conne ! Tu vas te calmer, oui ?](Varlar) La Rivoise se figea en reconnaissant Varlar, qui avait donc décidé de retrouver le chemin de sa demeure plus vite qu’elle ne l’avait anticipé. [Puisque je te dis que tu viens avec moi !](Varlar)

Il était hors de lui et ses projets n’avaient rien de pacifique. <Veut-il se venger de mon audace en s’attaquant à elle ?| se demanda-t-elle en se sentant prise de vertige à cette idée.|>(Aislinn) Il avait beau lui avoir offert l’adolescente deux ans plus tôt, légalement il en était toujours le propriétaire. Rien ne l’empêchait en théorie de l’amener sur un marché d’esclaves et de la vendre au plus offrant, voire de la donner au premier venu ou même de la tuer…

Le souvenir encore cuisant de la veille la retint de se ruer sans plus tarder au secours de son amie ; quand il avait levé la main au-dessus de sa tête pour la frapper, elle s’était sentie si impuissante et démunie qu’elle n’avait rien pu tenter pour l’arrêter. Il était hors de question que ce scénario se reproduisît, pas alors que la *survie* de Sanaa en dépendait. Elle se précipita donc jusqu’à l’âtre froid et poussiéreux de la pièce — Varlar ne s’en servait jamais — et saisit sans y réfléchir le tisonnier qui y trônait.

[*Aislinn !*| l’appela directement Sanaa et la panique qui transpirait dans sa voix fit l’effet d’une gifle à l’héritière.|](Sanaa)

Rassurée par le poids de son arme improvisée, elle quitta enfin cette pièce maudite pour se porter au secours de l’adolescente.

_______________________personnage______________________
                                                       Sanaa
______________________________________________________

Quand Aislinn avait refusé de répondre à la moindre de ses questions, Sanaa avait compris que sa confrontation avec Varlar n’avait pas dû aussi bien se passer qu’elles l’espéraient de prime abord ; lorsqu’elle avait vu le vieux Langecin débouler dans sa chambre comme un fou furieux, elle avait mesuré combien sa première intuition était à la fois juste et terriblement loin de la réalité Il avait saisi son bras avec une violence dont il n’avait jamais fait preuve jusque là et qui l’avait laissée complètement pantoise. Elle avait essayé de protester et de se débattre, mais il n’avait rien voulu entendre et lui avait même asséné un coup sec à l’arrière du crâne pour lui intimer le silence.

Ce simple geste avait fait remonter en elle des souvenirs qu’elle pensait avoir réussi à sceller loin dans son inconscient et elle s’était ratatinée sur elle-même.

[Ta maîtresse a dépassé les bornes,| lui expliqua-t-il d’une voix dure tandis qu’il la jetait presque dans le couloir devant lui.| Il est plus que temps que je lui rappelle que ses actions ont des *conséquences*. Allez, viens avec moi.](Varlar)

[Arrête,| protesta-t-elle en essayant de libérer son poignet.|](Sanaa)

Elle avait cru que les quelques heures qu’elle avait passées avec Sauveur l’aideraient à s’affranchir des remarques et des insultes de Varlar, qu’elle pourrait se sentir plus libre après ça… mais elle se rendait compte désormais qu’il la molestait combien elle avait été stupide. Elle ne savait pas quoi faire, sinon se débattre, mais il lui semblait que ses ruades ne faisaient aucun effet à son agresseur ; pis, il lui donna un violent coup de pied dans les jambes pour l’immobiliser et elle lâcha son premier cri sous le coup de la douleur.

[Espèce de petite conne ! Tu vas te calmer, oui ?| rugit-il hors de lui.| Puisque je te dis que tu viens avec moi !](Varlar)

[Non, non, non !| glapit-elle en se laissant cependant guider malgré elle.| Lâche-moi ! Arrête !](Sanaa) Puis, n’y tenant plus, elle leva la tête et hurla de toutes ses forces. [*Aislinn !*](Sanaa)

[Sanaa !| lui répondit l’héritière sur le même ton.|](Aislinn)

Elle s’approchait.

En entendant la voix de sa protégée, Varlar se figea, puis posa un regard un peu fou sur sa victime.

[Tu n’as rien à espérer de cette traîtresse,| lui asséna-t-il en relâchant légèrement sa prise.|](Varlar) Elle voulut essayer d’en profiter, mais seulement pour voir son ouverture lui échappait. Le Langecin recommença à la traîner derrière lui. Aislinn déboula dans leur dos en courant, le souffle court. Varlar ne lui fit même pas l’honneur de se retourner pour l’accueillir. [Ta petite rébellion s’arrête aujourd’hui et maintenant,| lui annonça-t-il avec mépris.| Quand je reviendrai, *seul* après m’être débarrassée de ta précieuse Vaanie, tu sauras que tu ne peux pas me défier comme tu l’as fait sans en souffrir les…](Varlar)

Il ne termina pas sa phrase ; Sanaa eut tout juste le temps de voir la pointe du tisonnier fondre sur lui et il couina comme un proc quand il sentit le métal froid lui entailler profondément la joue. Aislinn attrapa sa confidente par l’épaule et la tira vers elle pour qu’elle pût se mettre à l’abri de son tyran, qui se retournait en écarquillant les yeux.

[Je serai la dernière femme que vous sous-estimerez, Varlar,| lui promit-elle d’une voix vibrante de colère.|](Aislinn)

Sous le choc, Sanaa tituba de quelques pas en arrière, fascinée comme le Langecin par la pointe de l’arme d’Aislinn et par les goûtes carmines qui s’en échappaient.

[J’aurais pu vous tuer,| lui apprit Aislinn.|](Aislinn)

La Vaanie n’avait aucune idée de l’état d’esprit de la Rivoise en cet instant précis, d’autant qu’elle lui tournait le dos. Elle était cependant bien placée pour juger de l’effet que son assurance avait sur Varlar. Le péninsulaire avait plaqué sa sénestre sur sa plaie et donnait l’impression de ne plus savoir quoi faire face à la furie qui venait de le blesser.

[Ne sois pas ridicule,| finit-il par protester.| Un courant d’air te fait sursauter, jamais tu…](Aislinn)

Il ne termina pas sa phrase, car l’héritière Noblegriffon raffermit sa prise sur le manche de son arme improvisée, avant de lever la pointe du tisonnier à hauteur du visage de son adversaire, qui pâlit sous la menace.

[Digne m’a appris à me servir d’une épée,| lui apprit-elle d’une voix blanche.| Je ne pensais pas avoir à utiliser ses leçons un jour et encore moins contre vous, mais je vous le promets, Varlar : si vous touchez à nouveau à un seul des cheveux de Sanaa, je n’hésiterai pas.](Aislinn) Il voulut répondre quelque chose, mais elle lui coupa l’herbe sous le pied en haussant le ton d’un coup. [Assez ! Vous aviez mille et mille occasions de parler et je vous aurais écouté à chaque fois ; il est trop tard, désormais. *Je. sais. tout.*](Aislinn)

Ou en tout cas, elle en savait bien assez.

Réduite au rang de spectatrice de cette confrontation dont elle ne comprenait plus rien, encore ébranlée par le déchaînement de violence dont avaient fait preuve tant Varlar qu’Aislinn, Sanaa était perdue. La Rivoise dut s’en rendre compte, car elle posa un instant ses prunelles sur elle et tendit son bras libre pour lui caresser les cheveux avec douceur ; heureusement pour eux trois, Varlar ne se sentit pas l’audace de profiter de son court moment d’inattention pour tenter quoique ce fût.

[Ça va aller ?| lui souffla-t-elle avec une bienveillance infinie.|](Aislinn)

[Ne t’occupe pas de moi,| la pressa Sanaa sans quitter du regard le Langecin et le danger qu’il représentait.|](Sanaa)

[Bien sûr que si,| la contredit-elle en esquissant un rapide sourire avant de redevenir mortellement sérieuse.| Il t’a blessée ? Tu peux marcher ?](Aislinn) Et Sanaa d’opiner de répondre par quelques hochements de tête. [Très bien, alors ne reste pas ici. File à l’auberge de Kassim, je viendrai te rejoindre quand j’en aurai terminé avec *lui*.](Aislinn)

Elle voulut protester, mais quelque chose dans les yeux d’Aislinn l’en empêcha.

______________________personnage_______________________
                                           Aislinn Nobglegriffon
______________________________________________________

Une fois Sanaa partie, Aislinn reporta pleinement son attention sur Varlar, qui avait sagement préféré s’abstenir de commenter la fuite de la jeune esclave. La Rivoise aurait tellement voulu pouvoir suivre la Vaanie ; pour autant, elle avait conscience qu’il lui restait encore beaucoup à accomplir pour couper définitivement à son tuteur la moindre envie de se venger à nouveau et qu’elle ne pouvait plus attendre.

Ces découvertes de l’après-midi et cette altercation avait au moins l’avantage de l’avoir libérée de la culpabilité qu’elle avait pu ressentir.

[Nous savons tous les deux que tu n’es pas une tueuse, Aislinn,| finit-il par asséner.|](Varlar) Cet interlude lui avait permis de retrouver un peu de contenance. [Je dois cependant t’avouer que je ne te pensais pas capable de tout cela.](Varlar)

<Il essaie de reprendre la main|, comprit-elle en plissant les yeux, soupçonneuse.|>(Varlar) Ce ne devait pas être la première fois que ses manières et ses affaires le plaçaient du mauvais côté d’une arme ; qu’elle fut tenue par une adolescente devait être inédit, mais Aislinn ne doutait pas que cela changeait peu de choses en fin de compte. <Il joue ; l’or est l’étalon dont il se serre pour compter les points, rien de plus… Mère partageait cette vision du monde, fut un temps. C’est peut-être pour cela qu’elle lui a si souvent passé ses offenses.>(Aislinn)

Aislinn, cependant, n’était pas Katalina Noblegriffon ; et jamais la gardienne n’aurait fermé les yeux sur ce qu’il avait fait à Sanaa.

[Il faut que nous parlions — et pas seulement de ce que vous venez de faire. J’ai quelque chose à vous montrer,| lui intima-t-elle en lui indiquant de la pointe du tisonnier la direction de son bureau.|](Aislinn)

Elle avait pris sa décision. Elle ne devait plus attendre : elle devait le mettre hors d’état de lui nuire, à elle, mais aussi à ceux qu’elle aimait, aujourd’hui.

Surpris par ce soudain changement de ton et d’attitude, il ne chercha pas à protester et après un temps d’hésitation, haussa les épaules et se mit en marche. Elle lui emboîta le pas, sans se départir de sa précieuse arme. Elle lui semblait si légère — alors qu’elle ne l’était pas et que l’engourdissement de son bras finirait bien par la rattraper. Quand il poussa la porte de son antre, il lâcha un juron en constatant le désordre qu’Aislinn y avait mis pendant la journée. Sans se soucier de ce menu détail, elle le dépassa, dardant son regard sur les sol avant de ramasser un parchemin en particulier et de le parcourir rapidement des yeux. Elle reporta ensuite son attention sur Varlar.

[Au début du mois, j’ai visité les quatre entrepôts que nous possédons et le comptoir qui nous sert à vendre nos biens,| commença-t-elle.| Savez-vous ce que j’y ai trouvé ?](Aislinn)

[Ce qui me permet de t’acheter ces robes que tu te plais tant à porter?| suggéra acerbement le Langecin.| Je ne vois pas en quoi cela concerne notre différend actuel. À moins que tu espères encore que je vais tout te pardonner et fermer les yeux sur… ça ?](Varlar)

[Des esclaves, Varlar,| asséna-t-elle en ignorant la vilaine blessure qu’il cherchait à lui agiter sous le nez.| Trois esclaves pour chaque homme libre que nous employons, très exactement.](Aislinn)

Elle darda sur lui des prunelles révoltées. Elle avait eu beaucoup de mal à garder pour elle cette découverte, car elle ne savait pas comment elle devait la gérer. Cela ne sembla pas impressionner Varlar plus que cela, bien au contraire. Elle le sentit même se détendre et quand elle le vit commencer à ricaner un peu bêtement, elle comprit qu’il ne saisissait pas la mesure de ce qui était en train de se jouer. Comment pouvait-il croire qu’il s’en tirerait à bon compte, après ce qu’il avait fait subir à Sanaa ? Si Aislinn, dans ces circonstances entre toutes, prenaient la peine d’évoquer ces sujets avec lui, c’était qu’il devait être grave, non ?

[Voyons, Aislinn,| soupira-t-il en adoptant ce ton paternaliste qu’elle avait appris à détester récemment.| Nous avons déjà eu l’occasion d’en parler. Tu fais ce que tu veux de Sanaa, mais l’esclavage fait parti des traditions locales.](Varlar)

[Chez les autres !| explosa-t-elle.|](Aislinn) Elle redressa vivement le tisonnier vers lui et Varlar recula d’un pas en levant les mains devant lui. [Chez les autres, pas dans notre maison ! Et en même temps, quelle sotte je fais. Vous m’avez offert Sanaa après tout et jamais n’avez consenti à la libérer.](Aislinn) Sa candeur et naïveté lui faisaient honte. [Ne vénérez-vous pas les Cinq, à la fin ? Ne priez-vous pas la DameDieu ? Ou bien l’or est-il le seul Dieu que vous servez encore ?](Aislinn)

[Que veux-tu que je te dise ?](Varlar) Le peu de contrôle de soi qu’il avait réussi à rassembler recommençait à lui échapper. [Je ne fais pas les règles, je me contente de les suivre. La liberté coûte cher ici et si je souhaite dégager un tant soit peu de marges, je ne peux pas me permettre de…](Varlar)

[*Nous*. Nous ne pouvons pas nous permettre de faire preuve d’humanité si nous voulons nous enrichir… Je savais que vous me répondriez cela, sauf que nous faisons des profits,| le coupa-t-elle en attrapant un parchemin au hasard et en l’agitant devant elle.| Beaucoup, beaucoup *trop* en réalité ; c’est ce que j’ai découvert tout à l’heure et pendant que je me pavane dans ces robes que vous m’achetez, *nos esclaves* mangent à peine deux repas par jour car il y a deux ans, vous vous êtes rendu compte que cela ouvrait la voix à des économies substantielles ; et ils s’entassent les uns sur les autres pour dormir, parce que vous avez appliqué la même logique pour régler le moindre aspect de leurs existences !](Aislinn)

Cela dépassait de loin le seul sort de Sanaa.

Même s’il n’avait pas armé la main sur elle, Aislinn se serait peut-être armé de son tisonnier.

[Je fais ce qu’il faut pour t’offrir la vie que Katalina voulait pour toi,| argua le Langecin ; comme s’il était *autorisé* à jouer cette carte-là.|](Varlar)

[Je vous interdis de parler de Mère aussi familièrement,| asséna-t-elle en jetant le parchemin dans sa direction.| Si je suis certaine d’une chose, c’est que vous ne vous seriez jamais permis tout cela si elle avait été là pour garder un œil sur vous.](Aislinn)

[Petite, je connaissais déjà ta *mère* que tu n’étais pas encore née,| gronda le Langecin.| Si elle t’a confiée à moi, c’est bien parce qu’elle avait foi en mon jugement… Je découvre aujourd’hui que j’ai misérablement échoué dans cette seconde tâche, à mon grand désarroi !](Varlar)

[Oh, mais elle ne vous a jamais fait confiance,| objecta Aislinn.| Je dois d’ailleurs vous remercier, car sans vous, je ne l’aurais sans doute jamais compris… mais en lisant ses Mémoires, tout est devenu plus clair. Vous auriez dû vous y intéresser un tant soit peu avant de me conseiller de me les procurer.](Aislinn)

[Nous n’avons peut-être pas toujours entretenu les relations les plus cordiales qui soient,| convint-il en grimaçant à cause de la douleur que lui causait son entaille,| mais elle savait ce qu’elle faisait en te confiant à moi.](Varlar)

[En cela, je ne peux qu’être d’accord avec vous,| acquiesça-t-elle.| *Cet homme est un piètre marchand, mais un très bon escroc ; il vous fait fructifier une fortune en tarissant sa source, puis migre à la recherche d’un nouveau cours d’eau.* Je la cite. Mère se moquait bien de ce comptoir qu’elle vous a laissé fonder avec son nom, tant qu’il me gardait de la misère à laquelle elle s’était persuadée m’avoir condamnée quand elle a croisé ma route.](Aislinn)

[Elle a vraiment écrit ça ?](Varlar)

[Et d’autres choses, mais cela ne devrait pas retenir plus longtemps notre attention, car voyez-vous, mes découvertes ne se limitent pas à votre mépris absolu de la dignité humaine. Vous êtes aussi un idiot, qui pensez que défier le conseil marchand de Thaar en vous soustrayant à l’impôt du guet *est autre chose qu’une folie*.](Aislinn)

C’était la seconde pépite qu’elle avait déniché dans cet amoncellement de documents. Cette nouvelle accusation prit de court Varlar, qui voulut répondre quelque chose, mais sentit ses mots mourir dans sa gorge avant qu’il n’eût l’occasion de les cracher. <Cela doit vous rappeler de bien mauvais souvenir, n’est-ce pas ?| le provoqua-t-elle du regard.| Quand Mère vous a défait, il y a de cela bientôt vingt ans, est-ce que vous avez ressenti la même chose ?>(Aislinn)

[J’ai beaucoup de respect pour les princes marchands du conseil, Aislinn, mais cet impôt est une escroquerie sans nom. Il est exorbitant et...](Varlar)

[… et nécessaire ! Il est ce qui permettra un jour aux Thaarii de marcher dans toutes les rues de leur propre ville sans craindre de s’y faire molester ou pire. Vous êtes vous déjà promené hors des Soieries, à la nuit tombée ?](Aislinn) Elle marqua une pause, qu’elle mit à profit pour séparer ses mains et les reposer une nouvelle fois bien à plat pour se calmer. [En tout cas, c’est à cela qu’il devrait servir, mais là n’est même pas la question. Est-ce que vous vous êtes renseigné sur ce qui arrivait aux petits bourgeois comme nous qui sont découverts la main dans le sac à défier l’autorité du conseil ? Même moi je le sais !](Aislinn) Elle secoua sa dextre pour lui signifier qu’elle n’avait pas fini. [Oh, mais je connais déjà la réponse, car je ne vous crois pas assez courageux pour avoir pris un tel risque consciemment.](Aislinn)

[Il n’y a aucune chance que cela nous soit reproché,| protesta Varlar| Tout le monde cherche ce genre d’arrangement depuis la réforme du guet. Ils n’ont pas le choix… et moi non plus.](Varlar)

[Sombre buse ! Nous ne sommes pas des Vaanis, nous. Si le conseil doit faire un exemple, c’est vers nous qu’il se tournera le premier !](Aislinn)

Chaque seconde de leur confrontation qui passait ne faisait qu’accentuer un peu plus le mépris qu’elle ressentait depuis qu’elle avait fouillé ses archives. <Cet homme n’a aucun honneur et moins encore de moral. Si ses pairs se nourrissaient de bambins, il irait les chercher dans le ventre de leurs mères.>(Aislinn) C’était peut-être cela qui l’énervait le plus : Varlar n’assumait rien et blâmait le reste du monde. Elle était jeune et idéaliste assurément, mais elle voulait pouvoir se regarder dans le miroir sans la honte qu’elle ressentait en cet instant.

[Ce que vous avez fait aujourd’hui à Sanaa est impardonnable, ce que vous avez comploté pendant des années *en mon nom* — car vous avez toujours pris soin de vous auréoler du nom des Noblegriffon — est inqualifiable. Je ne veux plus vous voir. Vous allez retourner à Ys et vous ne reviendrez pas à Thaar tant que j’y vivrai, est-ce que c’est clair ?](Aislinn) Elle darda sur lui des regards brûlants en ajoutant. [Vous continuerez à percevoir une part des profits de *mes* comptoirs, ainsi que Mère vous l’a promis, mais vous n’aurez plus jamais votre mot à dire si quoique ce soit qui me concerne de près ou de loin… Et si je découvre un jour que vous cherchez à vous soustraire à cette injonction, je vous le jure Varlar : vous le regretterez. J’ai bien assez de cordes à mon arc désormais pour vous réduire au silence. Et si ce n’est pas moi qui le fait, alors ce sera le Vaisseau de la Voilée qui s’en chargera.](Aislinn)
_______________________personnage_______________________
                                           Aislinn Noblegriffon
______________________________________________________

__________________________date_________________________
Arcamenel 71 Verimios de l’an `13:XI`, à la tombée de la nuit, dans la chambre d’Aislinn.
______________________________________________________

Varlar avait quitté Thaar pour Ys moins de vingt jours auparavant, mais tandis qu’elle sentait sur son corps à moitié dénudé les caresses et baisés de Digne, Aislinn ne pensait plus du tout au vieux Langecin. Il fallait dire que depuis que Sanaa avait investi la chambre laissée vacante après son départ, sa relation avec le jeune Hadjaoui avait pris un tournant à la fois attendu et redouté par la Rivoise ; fort heureusement, son compagnon faisait en tendresse comme pour tout le reste preuve d’un savant mélange de passion, de patience et de retenue qui avait permis à la Rivoise de s’initier aux jeux du plaisir sans ressentir de pression particulière.

[Je t’aime, Digne Hadjaoui,| Susurra-t-elle au creux de son oreille tandis qu’elle l’aidait à retirer sa tunique ; et si elle en pensait chaque mot, elle le disait aussi en cet instant précis parce qu’elle avait bien compris quel effet cela faisait à son partenaire.|](Aislinn) Elle adorait voir son visage s’illuminer, les poils de ses bras se soulever et ses homoplates étaient secouées de léger frémissement d’anticipation. Elle se souvenait encore quand, presque timidement, il avait laissé ses mains s’aventurer sur son ventre et remonter sur ses seins pour la première fois. Elle avait senti son envie d’elle, mais sa peur aussi de la brusquer ; elle l’avait encouragé à continuer ses explorations en caressant ses épaules avant de l’embrasser. Ils n’étaient pas allés beaucoup plus loin ce soir-là, mais cela n’avait pas semblé le déranger. Quatre jours plus tard, elle s’était dénudée quasiment entièrement devant lui pour s’offrir, conquise, à ses cajoleries pleines de douceur. Le lendemain, elle l’avait poussé à se débarrasser de ses vêtements à son tour et elle avait pour la première fois promené son regard sur le corps d’un homme en proie au désir. Cette nuit-là, elle avait caressé timidement son sexe sans oser aller bien loin, avant de se laisser mener à son premier orgasme et d’en ressortir pantoise et déboussolée.

Chaque soirée passée en sa compagnie dans le secret de sa chambre lui ouvrait — *leur* ouvrait — de nouvelles perspectives, mais amenait aussi de nombreuses questions qu’elle ne savait à qui confier. Digne avait notamment commencé à s’aventurer dans son intimité avec son index lors de leur dernière « séance » et le plaisir induit par cette excursion n’avait pas réussi à supplanter une gêne persistante, presque une douleur qu’elle avait instinctivement tout fait pour lui cacher. Aislinn avait beaucoup changé depuis qu’elle avait échangé son premier baiser avec le Hadjaoui, mais elle restait tout de même fidèle à elle-même et elle ne voulait pas l’inquiéter plus que nécessaire. On l’avait prévenue que la première fois *serait* douloureuse et elle avait intériorisé cette idée. Tout ce qu’elle pouvait ressentir de négatif quand elle était avec son compagnon était donc *normal* et elle n’avait pas besoin de le troubler avec quelque chose contre lequel il était impuissant. Surtout, elle craignait que cela l’encourageât à repousser le moment où ils essaieraient ; or, si elle restait anxieuse à l’idée de l’accueillir en elle, elle s’était surprise à le vouloir aussi.

Par Arcam, elle en avait même *rêvé* à deux reprises.

[Digne ?| l’appela-t-elle et son cœur battait déjà la chamade.| Je crois que…](Aislinn) Elle darda sur lui des prunelles qui trahissaient tout ce qu’elle pouvait éprouver en cet instant précis : de l’amour, de la confiance, de la peur, mais du désir aussi. [Je crois que je suis prête, si tu l’es également.](Aislinn)

Elle ne doutait pas qu’il le fût, en l’occurrence.
__________________________personnage__________________________
                                            Aislinn Noblegriffon
____________________________________________________________


On lui avait dit qu’elle aurait mal.

Quand Digne avait posé son membre sur son intimité, elle avait instinctivement bloqué sa respiration, prête à accueillir la douleur qui devait accompagner les premiers mouvements du jeune homme ; puis, elle s’était souvenue qu’il était important qu’elle se détendît, alors elle avait essayé de faire exactement cela. Ensuite seulement elle avait opiné discrètement du chef et son amant avait compris qu’elle l’invitait en elle.

Très vite, elle sentit que quelque chose n’allait pas.

Digne ne voulait pas lui faire mal, il le lui avait assez dit et l’avait assez prouvé ; si bien qu’Aislinn devinait qu’il hésitait sur la manière dont il devait s’y prendre. Elle aurait souhaité pouvoir le guider, lui dire ce qu’elle ressentait, mais de son côté l’héritière avait clairement l’impression qu’il s’attaquait à une muraille qui ne lui offrait aucun passage. Elle essaya de se convaincre que tout était normal, que mille et mille femmes avant elle avaient vécu exactement la même chose et qu’elles avaient surmonté cette épreuve ; elle ferma les yeux, toute notion de plaisir oubliée, seulement concentrée sur la douleur qui ponctuait chacun des mouvements de l’Hadjaoui.

[Est-ce que tu es… dedans ?| demanda-t-elle timidement.|](Aislinn)

Elle n’osait pas regarder ou aller vérifier par elle-même ; sa question, cependant, dut trahir plus qu’elle ne l’avait anticipé et Digne prit l’initiative de se retirer d’elle à ce moment-là. Elle comprit alors que, non, leur première fois *n*’était *pas* normale et cela la mortifia. Dans le même temps, elle était soulagée, car cela voulait dire que la douleur allait lentement refluer.

Le Vaani fut le premier à parler et elle darda sur lui des prunelles surprises, car la voix de son compagnon était lourde de regrets et de culpabilité ; cela faisait trop écho à ce qu’elle ressentait elle-même et elle avait cru qu’il lui reprocherait quelque chose, pas qu’il se blâmerait lui-même.

[Digne…| souffla-t-elle en se redressant sur son coude.|](Aislinn)

Elle hésita sur les mots qu’elle devait prononcer et décida d’abord de se pencher sur lui pour l’embrasser. Leur baiser ne fut ni long, ni passionné, mais il était tendre et sincère.

[Tu as été parfait,| lui assura-t-elle avec beaucoup de douceur.|](Aislinn)

La Noblegriffon fit ce qu’elle savait faire de mieux ; elle rassembla tout ce qu’elle pouvait ressentir de négatif et de douloureux en cet instant et érigea une muraille tout autour, pour pouvoir se concentrer sur ce qui comptait vraiment pour elle : Digne. Elle refusait que leur tentative avortée minât le jeune homme.

[J’ai voulu aller trop vite et j’étais trop tendue,| expliqua-t-elle en lui caressant la joue.| La prochaine fois, les choses seront différentes. D’accord ?](Aislinn) Elle sourit et posa son front sur celui du Vaani. [Tu n’as rien à te reprocher.](Aislinn)
__________________________personnage__________________________
                                          Le Vaisseau de la Voilée
_____________________________________________________________

______________________________date____________________________
Panahos 67 Karfïas de l’an `14:XI`, dans la taverne des Hadjaoui.
_____________________________________________________________


La gardienne avait posé sa main sur sa poitrine, entre ses deux seins ; sous ses doigts, elle pouvait sentir le tracé irrégulier et boursouflé de sa cicatrice. <Je ne suis plus cette femme|, soupira-t-elle intérieurement et même si elle n’ignorait pas, en disant cela, qu’elle se mentait à elle-même, elle laissa retomber son bras le long de son corps.|>(Katalina) Elle expira l’air de ses poumons et les souvenances de sa vie avec l’héritière Noblegriffon fuirent son inconscient ; elle inspira profondément et d’autres prirent sa place. <Je ne peux plus être cette femme, mais je sais pourquoi je suis venue.>(Katalina)

[Tu vas bien ?| lui demanda une petite voix à côté d’elle.|](Illio)

Elle était arrivée à Thaar cinq jours plus tôt et, déjà, on l’avait confiée à un enfant. Illio ne l’avait pas quittée depuis. La gardienne s’accommodait bien de sa présence, car il était un jeune novice prometteur. L’affection qu’elle pouvait éprouver à son égard restait cependant très mesurée. Dans quelques ennéades tout au plus, elle reprendrait la route et lui demeurerait au service d’Ysmir Jhabar, le responsable du Sanctuaire dans lequel elle avait élu domicile.

[Ne t’inquiète pas,| le rassura-t-elle en tendant le bras dans sa direction.|](Katalina) Ses doigts rencontrèrent son oreille et, partant de ce premier repère, elle longea la courbe de son crâne avant de lui caresser les cheveux. Illio se dégagea bien vite, presque vexé qu’elle sous-entendît qu’il pouvait avoir peur.

[Je ne suis pas inquiet pour moi,| lui expliqua-t-il en croisant les bras.|](Illio)

Et la gardienne de sourire pauvrement. Illio n’était définitivement pas Aislinn ; la Rivoise avait vécu pour ces gestes d’affection qu’elle ne lui avait accordés que trop rarement.

[Ouvre-moi la voie,| lui ordonna-t-elle enfin.|](Katalina)

Quelques secondes plus tard, ils pénétraient ensemble dans la bâtisse chaleureuse et bruyante de Kassim, à la recherche d’Aimé Hadjaoui. Contrairement à Aislinn, la gardienne n’avait pas oublié le nom de l’auberge dans laquelle Aimé leur avait expliqué travailler. Pour la trouver, elle n’avait eu qu’à solliciter son concours au clergé de Tyra.

Après qu’elle eut fait quelques pas vers le centre de la pièce, guidée par Illio, les clients de Kassim commencèrent à la remarquer et ils se turent, un par un, pour darder sur cette étrange femme un regard mal à l’aise.

[Je cherche Aimé Hadjaoui,| déclara-t-elle avec lenteur et sa voix éraillée porta à peine jusqu’au cœur de la salle; très vite pourtant, son appel fut relayé par les habitués, qui se demandaient bien ce que l’ancien esclave avait pu faire pour attirer l’attention d’une telle sorcière.|](Katalina)
___________________________personnage_________________________
                                                          Sanaa
_____________________________________________________________

_____________________________date_____________________________
Pendant la neuvième ennéade de Favriüs de l’An `14:XI`, dans les Grands Sablons
_____________________________________________________________


Sanaa avait envie de hurler. De pleurer. De frapper quelqu’un. De se ruer dans l’arène. De fuir cet enfer. Depuis qu’elle avait appris que Sauveur devait affronter Urgoll’Ven, celui qu’on appelait à Thaar le Gobelin, mais qui régnait avec le reste de son Triumvirat sur les masses grouillantes du Puy d’Elda, Sanaa n’avait pratiquement pas fermé l’œil et n’avait guère plus mangé. Quelques jours avant les grandes mêlées, le gladiateur lui avait annoncé qu’il ne pourrait plus venir la retrouver pendant toute la période des jeux ; prise de court, elle avait préféré feindre l’indifférence plutôt que de faire face l’oppressante angoisse qui lui avait saisi la gorge à ce moment-là. Elle se souvenait avoir cherché à donner le change en pariant peut-être un peu mesquinement sur le fait que leur séparation serait brève ; Sauveur n’avait guère goûté la plaisanterie — l’adolescente avait blâmé son stress — et elle-même n’avait pas mis beaucoup plus de temps à se vexer. Ils s’étaient quittés les cœurs lourds des choses qu’ils s’étaient dites, mais surtout de celles qu’ils avaient tues.

Le principal regret de Sanaa, au moment où Sauveur avait levé ses deux lames vers son impressionnant adversaire, demeurait que sa boutade ne se fût pas révélée prophétique.

Elle n’était bien entendu pas la seule que les grands jeux de Thaar épuisaient, tant moralement que physiquement. Aislinn, Digne, Aimé — et Courtois, que Sanaa connaissait cependant à peine — était tous de plus en plus taciturnes, agressifs et anxieux à mesure que Sauveur surmontait les épreuves de Mamba. La Vaanie s’était vite refermée sur elle-même, chose à laquelle elle n’était pas coutumière ; elle avait tout fait pour fuir Aimé autant que faire se pouvait, car l’aîné de son amant savait ce qu’elle avait tû par ailleurs et, la voyant devenir l’ombre d’elle-même, avait essayé de percer sa carapace. Elle ne lui en tenait par rigueur — comment l’aurait-elle pu, quand tout ce qui lui importait était son bien-être, au point même de faire abstraction pour un temps des propres affres auxquels l’opiniâtreté de son frère le confrontait ? —, mais si elle avait érigé ainsi ses défenses, c’était pour se protéger d’un monde décidément par trop cruel pour elle. Elle aurait tout le loisir de les mettre à bas si Sauveur survivait.

*Si*.

Le soir de son impressionnante victoire pendant la désormais célèbre épreuve des hordes de morts, il était venu la retrouver, à la faveur de la nuit, dans sa chambre de ce qui restait en théorie la demeure de Varlar. Elle avait senti son cœur manquer un battement quand il l’avait poussée vers son lit, le regard brûlant et avait répondu à son baiser pressant avec une passion matinée de désespoir. Ce soir-là, elle s’était offerte à lui comme jamais elle ne l’avait fait auparavant ; comme si c’était la première fois ; comme si c’était sa dernière chance. Puis, elle l’avait supplié de provoquer sa propre défaite et avait vu son monde s’effondrer lorsqu’il avait refusé.

Il avait déjà tant fait qu’il avait l’impression de pouvoir toucher sa liberté ; encore quelques victoires et il pourrait s’en saisir, enfin.

Il *devait* continuer.

Elle s’était souvenu sa conversation avec Aimé et l’assurance qui avait été la sienne alors, quand elle avait affirmé à l’ancien esclave « qu’il n’y aurait jamais rien de sérieux » entre elle et son frère. Qu’ils ne faisaient que s’amuser.

Quelle sotte elle avait été.

Elle n’avait aucune idée de ce qu’elle ressentait pour Sauveur ; elle avait simplement conscience que la possibilité qu’il pût ne pas survivre *avant* qu’elle le découvrît la rendait folle. Quant aux sentiments qu’il entretenait, lui, pour elle… Elle ne préférait même pas y songer. Elle ne doutait pas qu’Aimé l’avait confronté aussi et elle aurait parié tout ce qu’elle possédait que ses mots à lui avaient été à l’image des siens.

[Tu seras libre, si tu meurs,| lui avait-elle répondu avec une lueur mauvaise dans le creux du regard.|](Sanaa)

En cet instant, elle avait décidé de lui en vouloir ; elle avait trouvé cela plus simple que de chercher à le comprendre. C’était la dernière fois qu’elle l’avait vu d’assez près pour humer son odeur, pour entendre sa voix, pour toucher sa peau, pour goûter ses lèvres.

Cette nuit-là, elle l’avait repoussée ; et finalement, elle était condamnée à le regarder se battre, non pas pour sa liberté comme il le lui avait affirmé, mais bien pour sa vie tout en sachant qu’une erreur de sa part pouvait les priver d’une chance de se réconcilier.

Elle ne croyait pas avoir été jamais si horrifiée ; elle était livide et tremblante et lorsque les armes du Gobelin trouvaient leur chemin jusque dans la chair de son amant, c’était comme s’il la blessait elle aussi. Ces maudits jeux étaient devenus une chaîne invisible qui la liait à Sauveur ; ils étaient des vents qui avaient soufflé sur les étincelles d’une passion dont elle n’avait pas eu le temps de découvrir les clefs et du brasier qui en résultait, elle goûtait la terrible morsure.

[Sauveur !| hurla-t-elle avec horreur quand le noirelfe porta le coup de grâce.|](Sanaa)

Ses pires craintes venaient de se réaliser, elle en était certaine : le gladiateur, transpercé par l’épée du sombre, mourrait.

Se dressant d’un bon sur ses pieds, elle oublia qu’elle n’était pas seule et commença à se frayer un passage entre les badauds devant elle. Elle vit à peine le vainqueur du combat dominer son adversaire défait et d’un coup de pied, le pousser à terre. Ce qu’elle contempla, c’était la tête de son amant frapper le sol et son corps ne plus bouger.

[Sauveur !| s’égosilla-t-elle à nouveau et cela eut l’heur de disperser les Vaanis autour d’elle.|](Sanaa)

Arrivée au bord des gradins, elle s’agrippa à la rambarde et se pencha autant que possible, uniquement pour être la témoin impuissante d’une scène plusieurs fois cauchemardée.

Les soigneurs évacuaient le vaincu sur un brancard.

Sauveur ne bougeait toujours pas.

Derrière elle, elle crut entendre quelqu’un l’appeler par son nom, mais elle n’y prêta aucune attention.

Elle pivota sur ses pieds et se rua vers la sortie la plus proche, le cœur battant la chamade et les joues baignées de larmes.

Quelques jours plus tôt, mue par un terrible pressentiment, elle avait mené sa petite enquête. Elle savait exactement où on était en train d’amener l’Hadjaoui et avait même été en mesure de découvrir comment elle-même pouvait s’y rendre depuis les tribunes. Elle fonça, sans la moindre hésitation ou sans se soucier ni de son souffle court ni des prunelles interloquées qui accompagnaient son passage.

Elle avait vaguement conscience de toucher au but quand quelque chose stoppa net sa progression. Son épaule agrippée par un garde moins aviné que les autres, elle manqua tomber misérablement ; il commença à lui expliquer qu’elle n’avait rien à faire ici, mais elle n’avait aucune envie de l’écouter ou de se laisser ralentir, alors elle lui griffa le dos de la main avec une violence dont elle ne se savait pas capable. Le Vaani lâcha un juron avant de l’envoyer valser par là où elle était venue. Sanaa s’étala de tout son long, les yeux écarquillés par la surprise et le choc ; puis, elle darda sur son agresseur un regard meurtrier et, après s’être relevé, lui sauta pratiquement dessus dans l’espoir de le dépasser.

Ce fut peu ou prou à ce moment-là qu’Aimé la rattrapa, bientôt suivi par Aislinn et Digne ; le garde lui aussi recevrait sous peu des renforts.

[Sauveur !| hurla Sanaa par-dessus la cohue, sans savoir elle-même si elle espérait encore entendre le gladiateur lui répondre…|](Sanaa)
___________________________personnage_________________________
                                              Aislinn Noblegriffon
_____________________________________________________________


Aislinn ne se souvenait pas avoir jamais vu Sanaa dans un état pareil. L’adolescente était à fleur de peau et réagissait comme une animale blessée. Par trois fois, elle chercha à se dégager de l’étreinte solide d’Aimé et l’héritière Noblegriffon n’était pas certaine de ce qu’elle aurait tenté si elle était parvenue à ses fins.

Décidant qu’elle avait assez tergiversé, la jeune Rivoise s’approcha de l’aîné des Hadjaoui et de son amie ; quand elle la vit s’avancer dans leur direction, Sanaa détourna le regard. Aislinn ne se laissa pas arrêter par ce qu’elle ne pouvait, en l’état, qu’interpréter comme une nouvelle marque de défiance. Elle posa sa dextre sur l’épaule de la Vaanie et l’attira vers elle. Aimé, comprenant son manège, la libéra de son étreinte et la seconde d’après, Sanaa se réfugiait contre la Noblegriffon sans plus chercher à faire barrage à ses sanglots.

[Tout va bien, Sanaa,| lui souffla Aislinn à l’oreille en commençant à caresser ses cheveux.| Je suis là.](Aislinn)

La Rivoise coula un regard désolé vers Digne ; elle aurait préféré pouvoir rester aux côtés du jeune homme, qui assurément avait besoin lui aussi de soutien. Il était clair que, s’il ne réagissait pas avec autant de violence, il avait été affecté par le spectacle cruel de son frère tombant sous les coups du Gobelin. Malgré tout l’amour qu’elle pouvait porter sur sa cadette, Aislinn ne pouvait pas s’empêcher d’en vouloir à Sanaa. Ces terribles moments de doutes et de crainte n’auraient pas dû être à propos d’elle.

Et en même temps, elle n’avait qu’à la sentir trembler dans ses bras pour comprendre que la Vaanie ne surjouait rien.

Sans cesser de caresser la chevelure si particulière de Sanaa de sa dextre, Aislinn tendit sa senestre vers Digne. L’Hadjaoui esquissa un de ses sourires que sa compagne aimait tant, puis il parcourut la distance qui le séparait des deux femmes avant d’entremêler les doigts aux siens. Elle l’attira à elle et posa son front contre le sien.

[Néera veille sur lui,| chuchota-t-elle et elle ne savait pas à qui elle s’adressait.|](Aislinn) Elle tourna ensuite son attention sur Aimé qui semblait avoir fini de négocier leur passage et surprit le regard avec lequel il les couvait tous les trois. Malgré toute l’horreur du moment, l’ancien gladiateur lui donnait l’impression d’être… heureux. Ou rassuré. <Je prendrai toujours soin d’eux|, aurait-elle voulu lui promettre en cet instant.| De vous tous.>(Aislinn) Quelque chose dans les prunelles de l’Hadjaoui lui laissa penser qu’il le savait, qu’il le comprenait. Leur échange muet prit cependant fin avant qu’elle n’eût eu l’occasion de trancher, car l’attention d’Aimé fut tout entière accaparée par le nouveau venu.

[Décidément, les Hadjaoui resteront toujours une pointe de flèche dans mes godasses,| déclara-t-il en guise de salutations en promenant son regard sur chacun d’eux.|](Hashar)

Il sembla tiquer en découvrant Sanaa, mais n’eut pas le temps d’ouvrir la bouche pour continuer. Aimé non plus n’eut pas l’occasion d’en placer une. La Vaanie s’échappa de l’étreinte d’Aislinn pour darder sur leur nouvel interlocuteur des prunelles impérieuses — mais encore rougies par ses larmes.

[Je veux le voir,| asséna-t-elle en faisant son possible pour paraître inflexible.| Sauveur, Noble Lame, peu importe le nom que vous lui donnez, je veux le voir.](Sanaa)

[*Nous* nous inquiétons tous pour lui,| intervint Aislinn sans parvenir à se retenir de foudroyer Sanaa du regard.|](Aislinn) Elle n’avait pas entendu la discussion animée qu’avait eue Aimé avec les gardes des Grands Sablons et n’avait donc aucune idée de l’identité de l’homme à laquelle elle s’adressait. Il ne fallait cependant pas beaucoup d’imagination pour deviner qu’il avait son mot à dire sur qui était autorisé à se rendre au chevet de Sauveur et qui ne l’était pas. Que Sanaa fût incapable de le voir et qu’elle mît en péril cette éventualité, pour elle-même, mais surtout pour les deux Hadjaoui présents, la dépassait complètement.

[Le gamin va s’en sortir,| lui répondit l’inconnu non sans esquisser un rictus amusé.| Ce n’est pas forcément passé loin, mais il ne gardera aucune séquelle de ce combat. Il est entre de bonnes mains, mais il va avoir besoin d’un peu de repos.](Hashar)

[Je veux bien rester ici avec Sanaa,| continua-t-elle en ignorant le regard de bête blessée de la principale concernée,| mais ses frères devraient avoir le droit de le rejoindre.](Aislinn)

Elle sut en disant cela que Sanaa ne lui pardonnerait pas cette « trahison », mais la Noblegriffon n’avait aucune envie de lui céder sur ce point. Peu importait la force de ses sentiments pour le gladiateur, Digne et Aimé étaient de son sang et ils n’avaient pas à pâtir de son manque de tempérance.

Dans le même temps, l’adolescente venait enfin d’obtenir la confirmation que son *compagnon* — si Aislinn en croyait les propos d’Aimé — était hors de danger et il sembla que cette nouvelle était tout ce dont elle avait besoin pour laisser s’effondrer ses défenses. Pourtant une main à sa bouche, elle inspira profondément. [Ô, Arcam…| gémit-elle faiblement avant de se jeter dans les bras d’Aimé en sanglotant :| Il va s’en sortir, il va s’en sortir, il va s’en sortir…](Sanaa) C’était comme si elle ne parvenait pas encore à s’en convaincre.

[Les soigneurs sont des énergumènes têtues,| intervint l’homme en haussant les épaules,| mais je peux bien ramener l’un d’entre vous avec moi sans risquer de m’attirer leurs foudres.](Hashar) Il posa ensuite son regard sur Aimé, lui laissant de facto le dernier mot sur l’identité du ou de l’heureuse élue.
__________________________personnage__________________________
                                                        Sanaaa
_____________________________________________________________


Surprise par la «violence» des paroles de Sauveur, Sanaa ne put qu’écarquiller les yeux, incapable de trouver quoi lui répondre dans un premier temps. Peut-être avait-elle péché par orgueil, mais elle n’avait pas imaginé qu’il la repoussât ; pourtant, il était là, face à elle, à lui expliquer qu’il aurait préféré à sa présence à elle celle de son aîné.

Les mots du gladiateur lui faisaient mal.

Elle ferma les yeux, inspira profondément pour essayer de reprendre un peu de contenance, puis chercha à ramener ses mains vers elle. <Je n’aurais pas dû venir|, était-elle forcée de constater la mort dans l’âme.| J’aurais dû écouter Aislinn et laisser ses frères se porter à son chevet.>(Sanaa)

Elle était sur le point de rompre une promesse qu’elle avait à peine formulé.

Lorsqu’elle desserra ses doigts, il se passa cependant quelque chose qui remit toutes les émotions corrosives qu’elle ressentait à ce moment-là. Peut-être était-ce à cause d’un éclat de douleur, ou par pur réflexe, ou bien il ne voulait vraiment pas qu’elle partît ; Sanaa ne se posa pas la question. Tout ce qu’elle nota, c’était que Sauveur, lui, avait subitement serré le poing et l’empêchait effectivement de se dégager.

[Tu es un idiot,| soupira-t-elle et sa voix chantait tout le soulagement qu’elle pouvait éprouver.| Je t’ai dit que je ne te lâchais plus, non ?](Sanaa)

Elle avait failli se parjurer ; elle ne commettrait pas deux fois la même erreur.

Elle se pencha en avant, posant son front sur celui de son amant et souffla, pour lui seul.

[Je reste avec toi.](Sanaa)
____________________________personnage________________________
                                                            Sanaa
_____________________________________________________________


Sa confession était des caresses pour son Souffle.

Alors qu’elle avait cru le perdre, qu’elle s’était vue rejetée, Sanaa se découvrait aimée ; c’en était trop pour l’adolescente, qui ne put retenir cette fois un sanglot dont elle ne savait plus trop ce qu’il charriait. <J’étais là|, voulait-elle lui répondre, mais dans sa bouche les mots se bousculaient tant et tant qu’ils lui nouaient la gorge et l’empêchaient de respirer.|>(Sanaa) Le pire, peut-être, était qu’elle ne pouvait pas *vraiment* profiter de la confession de son amant à sa juste valeur, car elle sentait toujours sur eux le regard d’Hashar. Comment ignorer, même l’espace de quelques minutes, que les dangers qu’avaient affronté Sauveur ces derniers jours étaient appelés à se dresseraient à nouveau sur sa route, à court ou moyen terme ?

Le public avait goûté au sang de l’Hadjaoui et c’était une saveur qu’il n’oubliait jamais.

[Aislinn…| souffla-t-elle si faiblement que seul Sauveur pouvait l’entendre.| Aislinn pourrait te racheter.](Sanaa) La Vaanie n’avait pas la moindre idée de la véracité de son propos, mais elle *voulait* qu’il fût juste. [Varlar ne peut plus l’en empêcher, elle est libre de faire ce qu’elle désire de son or.](Sanaa) Elle commençait à s’agiter. C’était comme si elle était une naufragée perdue dans une tempête infernale et qu’elle venait d’apercevoir un radeau de fortune. [Elle peut te sauver. Elle *doit* te sauver.](Sanaa)

Et la Vaanie de refouler aussi profondément que possible chaque souvenance, chaque réminiscence de conversations qu’elle avait pu avoir avec son amie sur l’état des finances des comptoirs Noblegriffon.
_____personnage______
Aislinn
_____________________

[Il est mignon,| commenta Sanaa tandis qu’elle s’asseyait à la table désignée par Digne.|](Sanaa)

[Hum ?| répondit distraitement Aislinn sans quitter du regard la porte par laquelle le jeune homme s’était éclipsé.|](Aislinn)

[Digne,| insista la Vaanie.| Il est aussi agréable à regarder que son nom est étrange.](Sanaa) Elle tendit la jambe pour toucher le tibia de son amie, qui ne faisait montre d’aucun effort pour écouter ce qu’elle disait. [En tout cas, *lui* te trouve à son goût.](Sanaa)

[Ana !|protesta Aislinn en rougissant.|](Aislinn)

Des deux adolescentes, Sanaa était — et de loin — la plus en avance sur ces questions là, bien qu’elle fut la cadette de la Rivoise d’une petite année. Varlar avait récemment dit que c’était dans son sang ; tous les Vaanii ne pensaient qu’à coucher, à boire et à manger et il n’était guère étonnant qu’une esclave aussi libre et bien traitée agît de la sorte. Aislinn avait protesté vertement contre ses propos, mais Sanaa avait convenu qu’il avait bien raison… et accentué encore un peu plus le trait depuis lors, sans doute car elle avait compris que son attitude agaçait le vieux Langecin. Sanaa détestait Varlar, surtout parce qu’il était le seul à la traiter encore comme une asservie.

[Quoi ? Pour une fois, je n’ai rien fait,| rappela doctement la Vaanie.|](Sanaa)

[Au contraire, tu en as *bien assez fait*,| soupira Aislinn.| De toute façon, c’est toi qui veut absolument dormir avec un homme avant la fin de l’année. Si tu trouves Digne si mignon, tu n’as qu’à le faire avec lui.](Aislinn)

[Dormir avec un homme,| se moqua gentillement Sanaa.| Je ne veux pas que dormir avec lui, Ellie ; surtout, je préfèrerai que mon premier partenaire soit consentant. Or, crois-moi, de nous deux, il n’avait d’yeux que pour toi. C’est à peine s’il m’a regardée.](Sanaa)

Elle ne semblait pas s’en soucier plus que cela.

==============

<Il était tel que je me souvenais de lui, et tout en même temps n’était en rien semblable|, confierait-elle plus tard à Sanaa.|>(Aislinn)
_________________________personnage___________________________
                                                     Sanaa
_____________________________________________________________


Sauveur était impitoyable.

Chaque mot qu’il prononçait était comme un défi qu’il lui jetait à la figure et la pauvre adolescente, exténuée, ne savait plus comment y réagir ; quand il ruina ses maigres espoirs de parvenir à le tirer de la fosse dans lequel la vie le maintenait, elle eut envie de hurler ; quand il mentionna la mort de son père et la promesse qu’il s’était faite, elle se tendit en retenant son souffle ; et quand il lui avoua toute la place qu’elle avait prise dans son quotidien, elle secoua piteusement la tête, vaincue.

[Tu es un idiot,| sanglota-t-elle seulement en luttant contre ce besoin viscéral qu’elle avait de l’enlacer.|](Sanaa)

Elle le sentait faiblir, déjà ; les séquelles de son combat le rappelaient à leur bon souvenir et elle fit son possible pour lui adresser un sourire rassurant pour l’apaiser quand son bras commença à trembler.

[Si tu veux t’allonger et fermer un peu les yeux, vas-y. Je serai là à ton réveil.](Sanaa)

Elle croyait chaque mot de cette nouvelle promesse qu’elle lui faisait.

Hashar, pourtant, devait émettre une opinion légèrement différente dès lors qu’il devint clair que le gladiateur avait été rattrapé par son état. Posant une lourde main sur l’épaule de la Vaanie, il l’invita à sortir, arguant que sa famille devait commencer à s’inquiéter. Elle eut beau protester, se débattre, hurler même le nom de son amant, rien n’y fit. Quand Noble-Lame rouvrit les yeux, Sanaa n’était plus là.

___________________________personnage_________________________
                                               Aislinn Noblegriffon
_____________________________________________________________

______________________________date____________________________
Kÿrianos 10 Barkios de l’An `14:XI`, devant le Sanctuaire de Tyra où réside Katalina Noblegriffon.
_____________________________________________________________


Les grands jeux de Thaar étaient terminés et Miradelphia connaissait désormais le nom de son « meilleur guerrier », en l’occurrence de la daedhelle Shyn'tae Vaen're. Aislinn n’avait pas assisté à son sacre. La Rivoise n’avait aucun goût pour les combats de gladiateurs et la seule raison pour laquelle elle n’avait suivi si assidûment le tournoi organisé par Mamba que pour soutenir au mieux Digne.

[Tu es sûre de toi ?| s’entendit-elle demander.|](Pierre)

L’héritière tourna la tête en direction de Pierre et lui adressa un sourire rassurant. Le Berthildois ne faisait aucun effort pour cacher son trouble et Aislinn n’avait aucun mal à imaginer son origine. Après tout, après trois longues années de séparation, ils étaient sur le point de retrouver Katalina Noblegriffon et s’ils avaient tous deux espéré sans se l’avouer toujours que ce moment arrivât un jour, ils ne se sentaient pas prêts à confronter la gardienne. Ils avaient tous les deux entendu les rumeurs qui avaient circulé à son sujet et comment, surtout, elle avait *tué un Roi*. De la femme qu’ils avaient connue et côtoyée pendant plusieurs années, il ne restait peut-être plus grand-chose… et en même temps, ils ne voulaient pas accepter cette éventualité.

[Je le suis,| lui assura-t-elle avec une tranquillité qu’elle était loin de ressentir.|](Aislinn)

[Nous ne devrions pas être ici,| soupira le pêcheur.|](Pierre)

La Noblegriffon posa la paume de sa dextre sur son épaule avec douceur et affection, sans vraiment croire que ce simple geste pourrait suffire à repousser trois années de tourments. Pierre était celui qui avait le plus souffert de la «defection» de la gardienne ; il avait très mal vécu, surtout, la décision de la Serramiroise de confier Aislinn aux bons soins de Varlar. La principale concernée avait regretté aussi ce choix qu’elle avait fini par comprendre, mais qui demeurait à ses yeux une terrible erreur, mais elle n’avait pas osé à l’époque s’opposer à la « dernière volonté » de sa mère.

[Nous n’avons jamais pu lui dire au revoir.](Aislinn) La Rivoise serra tendrement l’épaule de Pierre, avant de couler un rapide regard sur Sanaa. La Vaanie attendait, anxieuse, quelques mètres derrière eux qu’ils se décidassent à *agir*. [Tu dois le *sentir*, toi aussi,| insista-t-elle en reportant son attention sur son aîné.| Rien ne nous dit que pareille occasion se présentera à nouveau.](Aislinn)

[Elle est perdue pour nous, Aislinn,| s’entêta le Berthildois.|](Pierre)

La Rivoise lâcha son épaule, car elle savait qu’il avait raison. Il n’empêchait qu’elle ne reculerait pas. Elle ne pouvait pas obliger son vieil ami à la suivre, mais elle ne doutait pas qu’il le ferait. Après tout, il avait quitté Ys du jour au lendemain quand elle lui avait appris que la Noblegriffon se trouvait à Thaar. Se tournant vers Sanaa, Aislinn tendit un bras en direction de la Vaanie et lui esquissa un sourire rassurant auquel elle ne reçut qu’une vague grimace en réponse ; sa cadette finit cependant par s’approcher et entrelacer ses doigts aux siens.

[Je suis d’accord avec le pêcheur : nous ne devrions pas être là. Tu n’as pas besoin de cette femme,| lui asséna sa turbulente amie sans parvenir à détacher son regard de la lourde porte qui la séparait encore de la gardienne.|](Sanaa) Et d’ajouter, faiblement : [Tu m’as moi.](Sanaa)

Aislinn n’ignorait rien du peu d’amour que portait sa confidente à la Serramiroise ; il semblait que la Rivoise avait, au travers de ses récits, passé plus de temps à s’ouvrir sur les épreuves et les douleurs qui avaient ponctué sa relation avec Katalina Noblegriffon qu’à se remémorer les instants de paix et de bonheur.

[Je sais que vous pensez que je commets une erreur en vous amenant ici,| expliqua-t-elle une dernière fois,| mais c’est quelque chose que je dois faire…](Aislinn) Elle darda sur Pierre un regard déterminé. [Et je crois que tu le dois aussi, Pierre.](Aislinn)

Le concerné haussa les épaules.

À ce moment-là, la porte devant laquelle ils se tenaient depuis plusieurs minutes déjà s’ouvrit et un garçonnet d’une petite dizaine d’années surgit du Sanctuaire. Il marchait d’un pas fier et confiant, comme s’il était porteur de la mission la plus importante du monde. Avant même qu’il commençât à parler, Aislinn eut l’impression qu’elle le connaissait.

Il était elle, quelques années plus tôt, quand les matrones de Lwar lui avaient demandé de s’occuper d’*Andall*.

[Je m’appelle Illio,| se présenta-t-il après s’être planté face à la Rivoise.|](Illio)

[Bonjour, Illio,| le salua-t-elle avec émotion.| Je m’appelle Aislinn et voici Pierre et Sanaa.](Aislinn)

[Je sais,| répondit-il un peu pompeusement — ce qui ne manqua pas d’arracher un sourire amusé à l’héritière —,| je suis venu vous prévenir qu’*elle* vous attend.](Illio)

[D’accord,| acquiesça son interlocutrice avant de darder des regards inquisiteurs sur ses voisins.| Dans ce cas, nous n’avons aucune raison de traîner plus longtemps, n’est-ce pas ?](Aislinn) Comme ni Pierre ni Sanaa n’osaient réagir, elle ajouta : [Nous te suivons, Illio.](Aislinn)

Le Sanctuaire était plongé dans une pénombre étrange, si bien qu’il leur fallut quelques secondes pour s’y habituer quand on referma la grande porte derrière eux. Ils marquèrent un temps d’arrêt, un peu surpris.

[Elle s’est toujours méfiée du feu,| souffla Pierre à sa gauche et Aislinn savait qu’il avait raison.|](Pierre)

Dans la pièce dans laquelle ils s’étaient engouffrés, il n’y avait aucune trace de torches ou de braseros. La seule source lumineuse venait de bassins au fond desquels on avait déposé, ainsi que finit par le comprendre la Rivoise, des pierres phosphorescentes. Combien de fois sa mère lui avait-elle intimé le silence, quand elle était sur le point de lui confier un secret trop près d’une cheminée ? <La flamme est l’oreille du Guerrier|, avait-elle coutume de répéter.|>(Katalina)

[Aislinn ?| l’appela Sanaa et la Vaanie ne pouvait empêcher sa voix de trahir en partie la confusion qui l’agitait à ce moment-là.|](Sanaa)

Pierre aussi s’était figé, sous les regards d’incompréhension de deux autres. Illio, lui, commençait apparemment à s’impatienter [Vous avez peur du noir ?| finit-il par les interpeller à son tour.|](Illio)

[Ce n’est pas ça,| le détrompa Aislinn en esquissant malgré elle un sourire.|](Aislinn)

Elle allait préciser sa pensée, mais Pierre l’arrêta d’un geste, avant de tendre sa dextre devant eux pour lui désigner quelque chose. Surprise, l’héritière plissa les yeux dans l’espoir de s’acclimater un peu plus rapidement à la semi-pénombre qui régnait dans le Sanctuaire.

[Oh…| souffla-t-elle en sentant son cœur manquer un battement.|](Aislinn)

Ce fut le Berthildois qui, le premier, réagit. Ignorant complètement leur jeune guide, il s’avança vers le fond du temple, pour pénétrer dans l’eau froide du bassin qui faisait face à une impressionnante idole de la Voilée. Sans un mot, il s’agenouilla aux côtés de la gardienne, qui poussa un léger soupir à son arrivée, mais ne dit rien.

[C’est…| souffla Sanaa à son oreille sans parvenir à aller au bout de sa question.|](Sanaa)

Quand Aislinn opina difficilement du chef, elle crut l’entendre déglutir et la Rivoise serra ses mains dans les siennes pour la rassurer. Son émotion était réelle, mais elle n’avait pas peur. Pas le moins du monde. C’était même tout le contraire.

Deux ennéades plus tôt, les deux adolescentes avaient pris la mesure du fossé qui s’était insidieusement creusé entre elles pendant les premiers mois de l’An `14:XI` et elles avaient détesté cela. Il leur avait fallu plusieurs jours pour digérer cette découverte et au moins autant pour trouver la force d’essayer d’y remédier. C’était Sanaa qui, la première, avait esquissé un pas dans sa direction. Après sa défaite, Sauveur avait gagné un demi-mois de répit à tout le moins, mais l’Épée de Damoclès au-dessus de sa tête avait été par trop d’aspects bien trop réels pour que la Vaanie s’en satisfît. Le sommeil et l’appétit avaient continué de la fuir et elle avait eu désespérément besoin de trouver quelqu’un à qui se confier. Aislinn lui avait prêté une oreille attentive, mais très vite avait commencé à évoquer ses propres tourments.

La présence de Sanaa ce jour-là ne s’expliquait pas autrement que par le prisme de ces longues discussions qu’elles avaient eues et du fossé qu’elles venaient à peine de résorber, mais qu’Aislinn ne voulait plus jamais voir.

[Tu ne la rejoins pas ?| l’interpella Illio après quelques secondes de battement.|](Illio)

[Non,| répondit-elle paisiblement en secouant la tête.| Ils ont besoin d’un peu de temps pour se retrouver.](Aislinn)

[Mais ils ne se disent rien,| protesta l’enfant.|](Illio)

[Ils n’ont pas besoin de mots,| expliqua la Rivoise.|](Aislinn)

Cette situation étrange dura encore quelques minutes, puis Pierre poussa un profond soupir. [Je ne sais pas qui vous êtes,| expia-t-il avec difficulté,| mais vous n’êtes pas *elle*.](Pierre)

[Je ne le suis plus,| entendit Aislinn répondre la gardienne et ces quelques mots lui brisèrent le cœur plus qu’elle ne voudrait jamais l’admettre.|](Katalina)

[Son bâton ?| demanda le pêcheur avec gravité.|](Pierre)

[Il est perdu.](Katalina)

[Mais vous vous souvenez, n’est-ce pas ?| insista-t-il.|](Pierre)

[Je me souviens.](Katalina)

Il fallut encore quelques terribles secondes au Berthildois pour répondre : [Prenez soin de sa mémoire, dans ce cas.](Pierre)

[Je suis Mémoire,| énonça paisiblement la gardienne et ces quelques mots faisaient écho à un passé que Pierre comme Aislinn avaient toujours craint voir se répéter.|](Katalina) Aislinn murmura d’ailleurs, par-devers elle et au diapason de sa mère : [Mémoire n’oublie pas.](Aislinn)

Après un dernier regard pour cette femme qu’il aurait suivi au bout du monde, Pierre se redressa lourdement. Il darda des prunelles dures sur la statue de Tyra et ses traits se tordirent en une grimace douloureuse.

[S’il m’est donné de recevoir une ultime grâce des Cinq, que ce soit celle-ci : qu’en Votre Royaume, le moment venu, mon Souffle retrouve le sien.](Pierre)

Son Vaisseau à ses pieds releva légèrement la tête dans sa direction avant d’esquisser un pauvre sourire que la pâle lumière du Sanctuaire rendait plus triste encore qu’il ne l’était réellement.

[Ainsi soit-il.](Katalina)

Sans un mot de plus, le vieux pêcheur fit demi-tour, puis commença à s’arracher péniblement à l’eau froide du bassin, avant de marcher d’un pas lourd vers Aislinn, Sanaa et Illio. L’enfant ne comprenait pas exactement ce qui venait de se passer, mais il avait tout de même saisi toute la gravité de l’échange en grande partie muet entre la gardienne et son ancien « apôtre». Spontanément, Aislinn se porta à sa rencontre et le prit dans ses bras. Le Berthildois hésita quelques instants avant de lui rendre son étreinte.

[Tu avais raison,| lui glissa-t-il à l’oreille.| J’en avais besoin.](Pierre)

Et la Rivoise d’opiner doucement du chef, tout en craignant ce qui l’attendait. [Je ne peux pas croire qu’elle soit perdue,| souffla-t-elle avec émotion.|](Aislinn)

[Elle l’est, pourtant,| lui assura Pierre en l’écartant légèrement pour plonger son regard gris dans le sien.|](Pierre) Les quelques mots qu’il prononça ensuite étaient chargés d’une telle force qu’Aislinn ne put contenir le frisson terrible qui lui secoua l’échine. [*Méfie-toi de Mémoire.*](Pierre) Il lui sourit et ses lippes étaient lourdes de toute la tristesse qui lui étreignait le cœur, puis lui caressa affectueusement la joue. [Merci.](Pierre)

Et, avant qu’Aislinn ne pût répondre quoique ce fut, il la dépassa et sortit du Sanctuaire.

*C’était son tour.*

Elle se tourna vers Sanaa et son visage dut trahir son appréhension, car cette fois, ce fut la Vaanie qui se porta à son secours et Aislinn sourit en sentant la chaleur de ses mains sur ses doigts engourdis par l’air froid et humide du temple.

[Tu n’es pas obligée,| souffla l’adolescente et le sourire d’Aislinn s’accentua encore un peu.|](Sanaa)

[J’en ai besoin,| la détrompa-t-elle avant de se couler un regard vers la gardienne.|](Aislinn) Elle inspira profondément, puis se mit en marche… non sans chercher à attirer Sanaa à sa suite. Sa cadette se laissa faire sur quelques mètres, puis elle prit la mesure de la femme vers laquelle elle se dirigeait et, bien malgré elle, commença à renâcler. [J’ai besoin de toi,| souffla la Rivoise à son amie.|](Aislinn) Sanaa se figea, darda sur elle des yeux mi-accusateurs mi-surpris, avant d’opiner lentement du chef et de lui emboîter librement le pas.

[Bonjour… mère.](Aislinn)

C’était la première fois qu’elle l’appelait ainsi et ce mot, qu’elle avait fini par s’approprier, lui laissait tout de même un étrange goût en bouche. Pour elle, Meavh serait toujours Meavh.

[Bonjour, Aislinn,| la salua Son Vaisseau sans se retourner.|](Katalina) L’héritière vit les épaules de la Noblegriffon se soulever et s’abaisser avec lenteur, puis la gardienne ajouta : [Si tu es là, c’est qu’Aimé a échoué.](Katalina)

[Il a essayé,| lui assura Aislinn sans se laisser démonter,| mais il était clair qu’il n’y croyait pas.](Aislinn)

La tête de Son Vaisseau s’affaissa, ce que l’adolescente décida d’interpréter comme une invitation de son ancienne protectrice à la rejoindre. Elle enjoignit la Vaanie à la suivre d’un signe de menton, puis plongea un premier pied dans l’eau glacée du bassin. Sanaa ne put retenir un petit cri de surprise quand elle l’imita — à contrecœur — dans cette expédition. Ce son inattendu sembla déstabiliser Katalina, qui esquissa un vif mouvement de chef dans la direction de Sanaa avant de ramener lentement son visage vers la figure de Tyra. [Sanaa…| souffla-t-elle les yeux fermés ainsi que put le constater Aislinn qui était désormais arrivée à son niveau.| Je me souviens de toi.](Katalina)

Cette phrase sibylline perturba grandement la principale concernée, qui recula d’un pas en entendant la Noblegriffon prononcer son nom. Elle déglutit silencieusement et chercha un peu de courage en dardant un regard perdu sur Aislinn.

[Laisse-moi tout de même te la présenter,| répondit la Rivoise en esquissant malgré elle un sourire affectueux.|](Aislinn) Elle marqua une légère pause, mirant tour à tour les deux femmes. [Mère, je suis heureuse que tu puisses enfin rencontrer Sanaa…](Aislinn) Elle avait rêvé de dire les prochains mots pendant plus d’une année, sans espérer que l’occasion ne vînt jamais. L’émotion était donc palpable quand elle ajouta : [Ma sœur.](Aislinn)

Ces deux mots eurent des effets différents sur Sanaa et Katalina.

La première oublia instantanément tout le trouble qu’elle pouvait ressentir à cause de la présence si proche d’une gardienne dont on assurait qu’elle avait été le Vaisseau d’une Déesse pendant le Voile. Bien sûr, son aînée ne l’avait pas prévenue, moins pour ménager une quelconque surprise que parce que cette affirmation était lourde de sens et qu’elle avait attendu *cette* occasion pour qu’ils ne demeurassent pas lettre morte. La seconde ne sembla simplement pas comprendre. Elle ne réagit pas tout de suite et son impassibilité fit redouter à son héritière qu’elle ne goûtait pas la nouvelle. <Elle est* ailleurs*|, regretta Aislinn avec un début d’angoisse.|>(Aislinn) Cela ne dura pas aussi longtemps qu’elle l’avait craint, car Katalina ajouta vite : [Je ne me souviens pas de cela.](Katalina) Et cela pouvait vouloir dire tant et tant de choses que la Rivoise renonça à chercher à l’interpréter.

[Ce n’est pas grave,| lui assura-t-elle avant de s’agenouiller à côté d’elle.|](Aislinn) Elle ne se tourna pas vers Tyra, cependant, préférant à cette Déesse qui la peinait autant qu’elle respectait le visage torturé de Son Vaisseau. [Elle l’est.](Aislinn)

[Elle l’est,| répéta la gardienne avant de poser son regard aveugle sur Sanaa.|](Katalina)

[Je…| commença la jeune Vaanie, mais sa gorge nouée retenait ses mots.|](Sanaa) Comme lorsque Sauveur lui avait avoué ce qu’il ressentait pour elle, elle se découvrait muette. Elle avait passé des jours à regretter qu’à sa déclaration passionnée, elle n’eût opposé qu’une boutade maladroite. [Je le suis,| acquiesça-t-elle et elle dardait ses prunelles émues sur Aislinn.| De tout mon cœur.](Sanaa)

Et Aislinn de sourire, touchée par cette déclaration qu’elle savait incontrôlée de la part de sa turbulente amie. [Varlar a résisté tant qu’il a pu, mais il a finalement cédé à mes demandes,| enchaîna-t-elle en n’ignorant pas que les mots qui suivraient ébranleraient encore un peu plus l’adolescente.| Elle est libre, enfin, ainsi qu’elle aurait toujours dû l’être.](Aislinn) Un hoquet surpris ponctua cette déclaration, mais la Rivoise n’avait pas terminé. [Je t’en conjure, mère. Reconnais la, comme tu m’as reconnu moi, car si je suis une Noblegriffon, alors ma sœur l’est forcément aussi.](Aislinn)

[Aislinn !| s’étrangla Sanaa sans qu’il fût possible de savoir quelle émotion dominait les autres.|](Sanaa)

Et la Rivoise de lui intimer le silence, tendue comme la corde d’un arc en attendant une réponse.

[Il ne m’appartient plus de la reconnaître…| finit par avancer la concernée après ce qui sembla durer une éternité à ses interlocutrices,| car je ne suis plus cette femme. Tu es la dernière Noblegriffon. Pas moi.](Katalina)

[Alors la chose est actée,| déclara Aislinn avec détermination.|](Aislinn)

[Alors la chose est actée et à ceux qui me poseront la question, je dirai désormais ceci : Sanaa Noblegriffon est la sœur de ma fille.](Katalina)
__________________________personnage__________________________
                                            Aislinn Noblegriffon
_____________________________________________________________

_____________________________date_____________________________
Arkuisa 41 Barkios de l’An `14:XI`, dans la chambre d’Aislinn.
_____________________________________________________________



Pour la première fois depuis plus longtemps qu’elle ne parvenait à se souvenir, Aislinn se sentait bien. Étendue sur le côté, elle s’était lovée contre Digne qui de sa sénestre caressait la peau nue de son bras, tandis qu’il déposait de légers baisers pleins de tendresse dans le creux de son cou.

Les yeux fermés, la jeune héritière profitait seulement de l’instant.

Quelques jours après qu’elle l’eut enfin confronté là-bas dans son Sanctuaire, la gardienne de Tyra avait quitté Thaar sans laisser la moindre trace qui pût permettre de la retrouver. Cette nouvelle n’avait pas étonné Aislinn et, à la grande surprise de Sanaa et — pour ce qu’elle en savait — des Hadjaoui, elle n’en tirait aucune tristesse. Bien sûr, sa mère lui manquerait encore longtemps ; mais cette fois-ci, elle avait pu lui dire au revoir. C’était comme si sa courte entrevue avec la Serramiroise lui avait permis de tourner une page par trop lourde de son histoire.

Avec tendresse, celle qui était désormais l’*aînée* des Noblegriffon se retourna sur le dos pour darder sur son compagnon de cœur des yeux éperdument amoureux sur l’Hadjaoui.

[Tu es beau,| souffla-t-elle avec émotion.|](Aislinn) Et son regard de dessiner ses contours, sans gêne ni honte. Elle tendit la main vers sa joue, qu’elle caressa lentement, avant de laisser glisser ses doigts jusqu’à l’arrière de sa nuque pour l’attirer vers elle et lui voler un baiser passionné.

Ils étaient tous les deux à moitié dévêtis. Digne avait presque timidement écarté les pans de la robe de nuit légère d’Aislinn, qui ne tenait plus que grâce à une petite cordelette de lin nouée au niveau de sa taille. Elle l’avait quant à elle aidé à se débarasser de sa tunique juste avant qu’ils n’allassent se coucher. Cela faisait quelques jours à peine qu’ils avaient repris les explorations de leurs corps respectifs et ils étaient encore un peu gourds. Ce n’était plus aussi naturel qu’aux derniers soirs de l’année précédente, où ils avaient multiplié les jeux et les caresses avec la fougue propre à leurs âges et leur inexpérience. La douleur impérieuse et inéluctable qu’avait ressenti Aislinn lors des deux tentatives de Digne de se frayer un chemin en elle avait mis fin à tout cela et les Grands Jeux avaient dressé des murailles si hautes autour de l’*idée* même de leurs ébats qu’ils avaient recommencé tout juste à en reparler.

Quelque chose, pourtant, avait changé.

Leur baiser s’éternisait quand Aislinn se rendit compte qu’il n’était pas sans effet sur Digne. Elle pouvait sentir, chaque fois qu’elle bougeait légèrement ses jambes, sa verge rendue dure de désir. Elle le devinait essayer d’éviter son contact, car il savait que récemment encore, elle en était venue à le fuir plutôt que de s’y confronter. Elle le retint donc en l’enlaçant.

[Reste,| lui souffla-t-elle sans décoller ses lèvres des siennes.|](Aislinn)

Vingt jours plus tôt, Sanaa l’avait amenée voir une femme de sa connaissance, qui « s’occupait d’elle » — c’était ses propres mots — depuis qu’elle avait commencé à fréquenter Sauveur. La Vaanie, devenue sa sœur adoptive, lui avait expliqué avec beaucoup — trop — de détails comment s’étaient passés ses premiers ébats avec Sauveur et comment elle avait paniqué le lendemain à l’idée qu’elle pouvait peut-être porter son fils. C’était quelque chose qu’elle avait refusé de considérer la veille, mais qui s’était imposé à elle dès que le gladiateur l’avait laissée seule. Séparée de sa mère à sept ans, asservie depuis, Sanaa n’avait pas profité de la meilleure des éducations eu égard aux dangers et pièges des jeux du corps ; en cela, elle n’était pas si différente d’Aislinn. Elle s’était pourtant débrouillée pour trouver une sage-femme qui, après lui avoir servi un sermon d’anthologie, lui avait donné une décoction à boire cul sec. Le goût était immonde et l’effet encore pire, lui avait-elle raconté avec un détachement maîtrisé. Quant aux méthodes « préventives », elles étaient plus douces, mais Sanaa pensait que c’était fait exprès.

Tout s’était passé très vite ; si elle avait ne serait-ce qu’envisagé le déroulé de cette rencontre, Aislinn aurait voulu pour que Digne l’accompagnât, au moins afin qu’elle pût se réfugier dans ses bras une fois son épreuve terminée. Malheureusement, ni elle ni Sanaa n’avaient su l’anticiper. En quelques minutes, Aislinn avait du s’allonger sur un lit qui n’était pas le sien, remonter ses jupons sur son ventre, laisser une parfaite inconnue *osculter* son intimité — elle n’avait pas pu retenir un sursaut que la sage-femme l’avait *touchée*. La Vaanie avait été jusqu’à insérer sans délicatesse son annulaire, avant d’opiner pensivement du chef en se redressant.

C’était ainsi que la Rivoise apprit que son hymen était anormalement épais et qu’il était heureux que Digne n’eût pas plus « insisté » lors de leurs précédents ébats.

Le prix à payer pour pouvoir un jour jouir d’une vie sexuelle «épanouie» avait été « trois petites incisions », ainsi que l’avait résumé la guérisseuse. Sanaa avait écarquillé les yeux et Aislinn était devenue livide, mais cette dernière avait fini par accepter.

Le soir même, lorsqu’elle était retournée dans sa demeure, elle n’était plus vierge.

[Caresse-moi comme tu le faisais avant,| demanda-t-elle timidement à un Digne qui semblait indécis sur la marche à suivre.|](Aislinn)

Il ne savait pas les détails de la courte opération qu’elle avait dû subir. Elle n’avait pas osé — ou réussi — à lui décrire la scène ; Sanaa lui avait cependant avoué qu’elle avait été un peu plus précise dans le récit qu’elle lui avait fait. Les premiers jours, elle avait eu du mal à marcher et uriner, malgré les soins magiques de la sage-femme. La douleur lancinante avait laissé place à une gêne, qui s’était elle-même estompée au début de l’ennéade.

Elle se sentait prête à essayer à nouveau.

Ils devaient juste se guider l’un l’autre, sur cette route qui n’avait plus rien de naturelle chez eux.

__________________________personnage__________________________
                                            Aislinn Noblegriffon
_____________________________________________________________


Elle pouvait le sentir dans la manière dont il la touchait.

Elle le voyait dans la façon dont il évitait son regard.

Il avait peur.

Elle aurait préféré qu’il n’en fût rien, qu’il lui sourit et lui promit que tout se passerait *bien*, enfin. C’était beaucoup lui demander, elle en avait conscience, mais elle avait craint que, dans le cas contraire, son courage à elle la fuît et qu’elle repoussât, encore et encore, le moment où elle l’inviterait en elle. Ce n’était pas tant qu’elle en eût particulièrement *envie*, mais dans le même temps, elle ne pouvait pas imaginer s’être abandonnée aux soins étranges et douloureux de la sage-femme de Sanaa *pour rien*.

Elle décida donc de prendre les devants.

Elle l’écarta avec douceur, non sans avoir avant esquissé un sourire tendre pour le rassurer. À nouveau libre de se redresser, elle tira ensuite sur la fine cordelette qui maintenait son vêtement, qui glissa avec lenteur sur sa peau. Elle finit de s’en débarrasser d’un mouvement d’épaules, puis darda sur lui des prunelles déterminées. Les caresses de Digne avaient fait leur effet et elle avait senti son corps se réveiller à mesure qu’il s’approchait de ses centres de plaisir, mais il était clair qu’elle était moins guidée par le désir que par son envie de dépasser cet obstacle qui ne les avait que trop retenu.

Enfin dénudée, elle posa la paume de sa dextre sur le torse de son amant et y imprima une légère pression pour l’encourager à s’allonger sur le dos. Elle se pencha ensuite au-dessus de lui et ses cheveux blonds cascadèrent autour de son visage. Elle le dévora du regard, puis vint cueillir ses lèvres ; dans le même temps, sa sénestre entamait un voyage jusque sous son ventre. Elle le sentit réagir quand ses doigts se refermèrent sur son sexe et ne put s’empêcher de sourire, un peu mutine.

Et l’héritière de constater, au creux de sa peur, que son désir continuait de poindre.
__________________________personnage__________________________
                                             Aislinn Noblegriffon
_____________________________________________________________


Ses caresses, comprit Aislinn, obtenaient l’effet escompté. Son regard et ses gestes et sa respiration le trahissaient : Digne était rattrapé par un désir qu’il avait par trop longtemps muselé.

L’héritière, pour sa part, continuait d’avancer sur le fil d’une crainte tenace ; son corps répondait aux attentions de son compagnon, mais c’était comme si elle ne percevait que des échos de plaisir. Cet effet étrange s’accentua encore quand Digne la fit basculer sur le dos et commença à embrasser son ventre avec envie. Elle comprit très vite quels étaient ses projets, mais contrairement à ce que le jeune homme espérait sans doute, la perspective de le sentir goûter son intimité ne l’enthousiasmait guère. Cela pouvait sembler contradictoire, d’autant que c’était elle qui lui avait demandé qu’il la caressât « comme avant », mais elle savait ce qui viendrait *après* et ses pensées étaient tellement accaparées par ce moment fatidique qu’elle n’imaginait pas être incapable d’apprécier *vraiment* quoique ce fût.

Son corps, pourtant, devait lui démontrer son erreur ; dans un premier temps, la Noblegriffon ne sentit rien qui fût comparable aux décharges de jouissance qu’elle avait connues pendant ses ébats avec Digne. Le Vaani se montra patient. Le temps dut d’abord lui paraître un peu long, surtout qu’un étrange silence s’était abattu dans la pièce. Il put cependant se rassurer quand la respiration d’Aislinn commença à s’accélérer ; ses efforts portaient enfin leurs fruits. Quelques secondes plus tard, un premier gémissement trouva son chemin jusqu’aux frontières de ses lippes et elle glissa ses doigts dans ses cheveux et accompagna ses mouvements de tête.

Puis, pour une raison qui n’appartenait qu’à lui, Digne prit la décision de ne pas s’aventurer plus avant vers la libération de son plaisir. Aislinn darda sur lui des prunelles où la surprise le disputait à la frustration… puis elle se rappela les enjeux que les caresses de son amant avaient un temps éclipsés.

[Ne va pas trop loin, d’accord ?| lui demanda-t-elle sans parvenir à complètement masquer son appréhension.|](Aislinn) Elle doutait que le benjamin des Hadjaoui eût nourri d’autres projets, mais elle trouvait néanmoins rassurant d’énoncer ses volontés à voix haute. [Je te dirai quand continuer…](Aislinn)

___________________________personnage_________________________
                                              Aislinn Noblegriffon
_____________________________________________________________



Aislinn pouvait enfin le sentir en elle.

D’abord, l’adolescente avait cru que cette tentative ne serait en rien différente des précédentes. Malgré tous ses efforts, elle n’avait pas pu s’empêcher de se tendre et, après avoir bloqué sa respiration, elle avait fermé les yeux pour mieux accueillir la douleur qu’elle pensait inévitable. Cela lui avait rappelé sa conversation avec la sage-femme de Sanaa, après qu’elle eût terminé son opération. Elle avait été mise en garde précisément contre ce genre de réactions et elle s’appliqua donc à user de l’astuce que lui avait confiée la guérisseuse. D’abord, elle se força à inspirer profondément, puis expira lentement ; elle accompagna sa respiration de légers mouvements du bassin qui devaient l’aider à se détendre.

Digne, lui, avait dû sentir son trouble, car il avait attendu patiemment qu’elle l’autorisât à poursuivre plus avant son exploration. Après quelques secondes supplémentaires pour rassembler son courage, elle avait opiné du chef pour lui communiquer son accord; cette fois, elle avait bien pris garde de ne pas leur compliquer inutilement la tâche. Tandis qu’il progressait, il eut été inexact de dire que l’héritière des Noblegriffon ne ressentait aucune douleur, mais cela n’avait rien de commun avec leurs deux précédentes tentatives. En l’occurrence, il s’agissait plus d’une gêne diffuse, sous la forme d’un tiraillement un peu lancinant. En somme, si peu de choses…

La jeune Rivoise était soulagée.

Elle vit dans le regard de Digne que l’Hadjaoui partageait son sentiment et son sourire l’inonda d’une chaleur bienvenue. Il était si beau, penché sur elle, *en elle*. Quand il l’embrassa, elle lui rendit son baiser avec une fougue nouvelle, tandis qu’elle le sentait bouger. Avant longtemps, elle ressentait des choses qu’elle n’avait fait qu’explorer avant et son bassin commença à accompagner timidement les va-et-viens de son compagnon.

[Je t’aime,| se surprit-elle à gémir en enroulant ses bras autour de son cou.|](Aislinn) Elle l’attira contre lui, sans se rendre compte qu’elle gênait ses mouvements. Elle voulait le sentir contre elle. *À elle*. [Je t’aime,| lui répéta-t-elle au creux de son oreille désormais à porté.|](Aislinn)

Et la Noblegriffon de commencer à lui mordiller le lobe, sans trop savoir pourquoi, simplement guidée par ces sensations nouvelles qui l’envahissaient et supplantaient même son soulagement. Ils étaient deux corps qui se mouvaient à l’unisson ; maladroitement encore, eu égard aux pratiques aventureuses des Vaanis, mais en harmonie. Elle n’avait plus besoin de lui dire quoi faire, elle pouvait lui montrer. Quant à Digne, Aislinn avait l’impression qu’il s’était débarrassé d’un coup de l’encombrant manteau de reproches qui l’avait paralysé ces dernières ennéades.

[Oh, Digne,| gémit-elle à nouveau.|](Aislinn) Elle ne l’attendait plus, mais elle la sentait venir, cette vague de jouissance dont lui avait parlé Sanaa. Sa respiration était saccadée et, cédant à l’égoïsme de son plaisir, elle oublia un temps celui du Vaani pour se concentrer sur ce qu’elle ressentait. Elle finit par s’arquer, tandis que son corps tout entier était traversé par des spasmes délicieux. Quand elle rouvrit les yeux, le souffle court, elle darda sur Digne des prunelles amoureuses, avant de se rendre compte qu’elle n’avait pas la moindre idée encore de ce que *lui* avait vécu.

Si les descriptions de Sanaa sur les suites de l’orgasme masculin étaient exactes, elle risquait cependant de le découvrir très vite.
__________________________personnage__________________________
                                             Aislinn Noblegriffon
_____________________________________________________________


_____________________________date_____________________________
Panahos 13 Verimios de l’An `15:XI`, dans la demeure des Noblegriffon.
_____________________________________________________________

Alors que l’An `15:XI` touchait à sa fin, les jeux de Thaar semblaient bien loin. Quand la fatigue rattrapait Aislinn, l’héritière des Noblegriffon repensait à ces terribles ennéades durant lesquelles elle et sa famille — car c’était ainsi qu’elle considérait déjà à l’époque Sanaa et les Hadjaoui — avaient vécu au rythme de cette mascarade barbare où quelques-uns s’étaient affrontés au péril de leurs vies pour le plaisir du plus grand nombre. Elle se souvenait de l’adolescente qu’elle était à ce moment-là, des épreuves qu’elle traversait et comment elle était persuadée alors que son monde pouvait sombrer à chaque instant.

[Tu n’as encore rien mangé ce soir.](Sanaa)

Aislinn fronça les sourcils et darda sur le reflet de Sanaa en face d’elle un regard faussement agacé ; la Vaanie, bien sûr, ne le laissa pas impressionner. Elle agita la brosse qu’elle tenait dans sa dextre, comme si ce geste constituait en soi un argument imparable, avant de recommencer à s’occuper des cheveux de la Rivoise.

[Tu ne dors pas assez non plus,| insista l’adolescente sans interrompre cette fois son ouvrage.|](Sanaa)

[Je rencontre Salem demain,| soupira Aislinn.|](Aislinn)

Le Thaari était un de ses partenaires commerciaux réguliers ; elle appréciait sa franchise et sa relative honnêteté, mais elle aurait préféré qu’il ne cherchât pas toujours à négocier le moindre prix ainsi qu’il en avait l’habitude. Chaque fois qu’il le pouvait, il faisait systématiquement d'une souris une incroyable montagne ; c’était comme un jeu pour lui, il fallait qu’il essayât, même s’il savait que la Noblegriffon cédait par trop rarement.

[Salem ne sera pas content si tu t’évanouis devant lui,| souleva Sanaa.| Il pourrait même le prendre relativement mal.](Sanaa)

[Je ne m’évanouirai pas,| objecta Aislinn en haussant les épaules.|](Aislinn)

[Ça, je n’en mettrai pas ma main à couper.](Sanaa) La Vaanie fit retomber ses bras le long de son corps, puis darda elle aussi son regard sur le miroir auquel elles faisaient toutes deux faces. [Tu devrais me laisser y aller.](Sanaa)

[Tu ne sais même pas de quoi nous devons parler,| s’agaça la Rivoise.|](Aislinn)

[Je le saurais si une certaine héritière de ma connaissance faisait un peu plus d’efforts en ce sens.](Sanaa)

Elles avaient déjà eu cette conversation plus de fois qu’Aislinn ne pouvait les recenser. Sanaa avait beau être officiellement sa sœur et possédait autant qu’elle les deux comptoirs qui portaient leur nom, force était de constater que c’était son aînée qui s’investissait le plus dans l’entreprise familiale. Dans les ennéades qui avaient suivi son adoption, la Vaanie avait renoué avec la sensation grisante d’être libre et devait par ailleurs s’habituer à l’idée qu’elle était, désormais, une Noblegriffon ; Aislinn, elle, s’était laissée enchaînée par les tâches qui avaient jusqu’ici échu à Varlar. Le temps que Sanaa comprît de tout cela, la Rivoise portait déjà les comptoirs à bout de bras. Les deux sœurs avaient d’abord cru qu’elles pourraient travailler de concert, avant d’être rattrapées par la réalité : Sanaa rechignait à s’épuiser autant qu’Aislinn, qui hésitait en conséquence à confier davantage de responsabilités à sa cadette.

[Tu peux venir, si tu veux,| proposa la susnommée héritière.|](Aislinn) Elle marqua une pause, puis ajouta : [Salem t’aime bien.](Aislinn)

[Je suis sa Noblegriffon préférée,| abonda la Vaanie en esquissant un rictus amusé.| C’est ce qu’il n’arrête pas de me répéter, en tout cas.](Sanaa) Aislinn sourit en retour et opina doucement du chef, ce qui sembla satisfaire sa sœur ; Sanaa baissa son regard sur la chevelure blonde de son aînée et recommença à les coiffer. [Sauveur pense qu’Aimé et Digne rentreront bientôt de Baaz’Hima,| annonça-t-elle après deux bonnes minutes de silence.|](Sanaa)

À la mention des deux Hadjaoui, le cœur de la Rivoise se serra ; à son grand désarroi, sa relation avec Digne avait pris un tour… préoccupant. Ils étaient tous les deux débordés ; lui multipliait les voyages avec son frère aîné, dans l’espoir de retrouver traces de ses sœurs et des épouses de son père ; elle ne parvenait plus à décrocher vraiment son esprit de son commerce.

[D’accord,| répondit-elle prudemment…|](Aislinn) Tout en sachant que cette réaction trahissait son malaise.

[Les choses ne s’arrangent pas entre vous ?| lui demanda Sanaa en posant sa brosse sur la petite table à côté d’elle.|](Sanaa)

[Ce n’est pas ça,| soupira Aislinn.|](Aislinn) Elle ne parvenait pas à admettre que son couple battait de l’aile ; elle savait qu’elle était amoureuse de Digne et ne doutait pas de ses sentiments à lui. Simplement, quand ils étaient ensemble, leurs soucis respectifs finissaient toujours par les arracher à l’instant présent. [Nous avons juste besoin d’un peu de temps pour…](Aislinn) Elle haussa les épaules. [Nous avons besoin de temps.](Aislinn)

Penser à Digne la démoralisait ; elle n’avait qu’une envie, se lover dans ses bras et se nourrir de sa chaleur. Quand il était absent, elle ne parvenait pas à se départir totalement d’une dérangeante sensation de vide ; quand il était présent, elle culpabilisait de ne pas pouvoir simplement profiter de lui. Un mois après la fin des grands jeux, elle avait brutalement mis ses comptoirs en accord avec ses principes moraux, en libérant sans condition tous les esclaves que Varlar avait achetés. À ceux qui en avaient bien voulu, elle avait proposé un contrat honnête ; la plupart avaient accepté, plus ou moins forcé par les circonstances. Il avait fallu presque un an de travail acharné à la Noblegriffon pour repousser durablement le spectre de la faillite, qui n’aimait rien moins que de s’abattre sur les commerçants de bonnes intentions. À cette occasion, Aislinn avait découvert ce que signifiait « être responsable d’autrui ».

L’idée qu’une erreur de sa part pouvait précipiter des dizaines de malheureux dans la misère l’avait tenue éveillée plus d’une nuit.

[Tu dis toujours ça,| souligna Sanaa en fronçant les sourcils,| mais il faudra bien à un moment que tu acceptes que les journées n’ont que les heures que les Cinq font, pas plus.](Sanaa)

Aislinn avait beau savoir qu’elle avait raison, il lui manquait encore une solution concrète qui lui permît de ralentir la cadence sans avoir l’impression de trahir quiconque. Vaincue, elle secoua piteusement la tête, avant de renverser son chef en arrière pour regarder la Vaanie directement et plus par le prisme d’une surface réfléchissante.

[D’ailleurs… Tu m’as bien dit que tu avais invité Sauveur et… Courtois… |— elle avait prononcé le nom de l’ancien gladiateur d’une toute petite voix, car malgré toute sa bonne volonté, ce Hadjaoui-là continuait à lui faire un peu peur —| à dîner ?](Aislinn)

[C’est ça,| acquiesça Sanaa en souriant,| mais nous avons encore assez de temps pour nous préparer à les accueillir sans avoir besoin de nous presser.](Sanaa)

[Tu me promets que vous n’allez pas vous éclipser avant la fin de la soirée pour…| commença Aislinn avant de grimacer.|](Aislinn)

[Je ne te promets rien du tout,| répondit une Sanaa goguenarde.|](Sanaa)
___________________________personnage_________________________
                                               Sanaa Noblegriffon
_____________________________________________________________


Le souffle court, Sanaa ne répondit pas tout de suite aux plaisanteries pleines de légèreté de son compagnon, se contentant d’abord de le couver d’un regard où l’amour le disputait à la gourmandise. Eussent-ils disposé d’un peu plus de temps, sans doute aurait-elle empêché l’ancien gladiateur, devenu mercenaire depuis sa libération, de se recouvrir. Il leur faudrait cependant attendre un petit peu, étant donné qu’à quelques mètres à peine patientaient Aislinn et Courtois. Aussi, lorsque Sanaa eut repris un semblant de convenance, elle se laissa tomber du haut de son trône improvisé, puis tendit le bras pour saisir une pièce d’étoffe abîmée que les serviteurs de la demeure — *ses* serviteurs, d’une certaine manière — utilisaient pour nettoyer et éponger les plans de travail de la cuisine.

[Ton frère ne fait aucun effort pour la ménager,| fit-elle remarquer en esquissant une moue mi-figue mi-raisin.|](Sanaa)

Elle appréciait Courtois, qui le lui rendait plutôt bien. Son côté bourru amusait beaucoup la jeune — désormais — Noblegriffon, qui l’asticotait régulièrement sur ses manières. À l’inverse, Aislinn restait invariablement sur la défensive et la mesure dès qu’elle devait lui parler, ce qui ne semblait pas au goût du concerné ; Sanaa était prête à parier qu’il se vengeait en se jouant de son aînée.

Elle allait pour ajouter quelque chose, mais sentit à ce moment-là un liquide poisseux commencer à se répandre le long de ses cuisses ; elle souleva les jupons de sa robe de sa sénestre et entreprit d’essuyer le mélange de leurs sécrétions, non sans jeter un regard un brin agacé à l’Hadjaoui. Le pauvre n’y était pas pour grand-chose, mais elle ne manquait jamais une occasion de pester devant l’injustice flagrante dans laquelle les plaçaient leurs ébats improvisés, d’un point de vue purement hygiénique — d’autant qu’elle n’était pas à l’abri d’une seconde coulée plus tard dans la soirée ! De son côté, Sauveur s’était mis en quête de l’objet initial de leur venue : le dessert. Le cuisinier qu’Aislinn et Sanaa employaient régulièrement depuis le début de l’année avait eu la gentillesse de leur préparer un plateau de pâtisseries au miel.

Quand il revint vers elle avec son précieux changement, elle se rappela d’un coup *pourquoi* elle avait tenu à l’accompagner en premier lieu ; instantanément, elle sentit son ventre se nouer. Il l’invita à la suivre et elle opina raidement du chef, avant de prendre une grande inspiration et d’énoncer, le plus sereinement du monde : [Je porte notre enfant.](Sanaa)

Elle avait longuement hésité sur la meilleure manière de lui annoncer la nouvelle et avait préféré opter pour une introduction toute en sobriété ; elle comptait sur la première réaction du concerné pour lui permettre d’affiner son discours.
_________________________personnage___________________________
                                           Sanaa Noblegriffon
_____________________________________________________________


Aux questions de Sauveur, Sanaa opposa dans un premier temps un silence lourd de non-dits ; l’ancien gladiateur, ainsi qu’elle l’avait anticipé, ne semblait pas transporté par la nouvelle et déjà cherchait qui ou quoi blâmer. Quelque part, ses quasi-accusations l’agaçaient. <Ou quoi ?| avait-elle envie de le défier… sans pour autant oser esquisser ce pas, car c’eût été commencer une guerre qu’elle n’avait pas les moyens de gagner.|>(Sanaa) De fait, il y avait un « ou » dans son histoire.

Si la Vaanie était enceinte, c’était parce qu’elle l’avait voulu.

Si Sauveur le découvrait si tard, c’était parce qu’elle l’avait choisi.

Elle avait souvent essayé d’aborder le sujet avec lui, sans que jamais la question fatidique ne franchît ses lippes serrées par l’angoisse d’un refus. Sanaa ne savait plus quand l’idée l’avait effleurée la première fois, mais les ennéades passant, ce qui était né comme une pensée fantaisiste s’était lentement mué en une véritable obsession ; il était clair à ses yeux qu’elle était la victime des chants d’Arcam, mais elle ignorait desquels exactement. Voulait-elle être mère ? Avec son asservissement, son lien avec son sang avait été rompu au delà du réparable et quand bien même était-elle sincère quand elle appelait Aislinn « sœur », n’éprouvait-elle pas une jalousie évidente lorsqu’elle était en présence des quatre Hadjaoui ? Y voyait-elle un nouveau moyen de posséder un peu plus de Sauveur ? Même si elle n’en montrait rien ou presque, elle demeurait profondément marquer par les interminables minutes durant lesquelles elle avait cru l’ancien gladiateur mort ; devenu épée-louée au service d’une des figures les plus emblématiques, puissantes et — fatalement — enviées des principautés, il était loin d’être « hors de danger ».

De son désir d’enfant, elle ignorait donc les racines ; elle savait tout, par contre, des raisons qui l’avaient poussée à n’en rien dire à son compagnon.

Pendant plus de sept ans, ses désirs avaient été soumis au bon vouloir d’autrui, toujours, si bien que *demander* était depuis une gageure insurmontable. Quand elle avait compris qu’elle était prête à assumer seule son envie et ses conséquences, elle avait choisi de se taire. Qu’il s’enthousiasmât en apprenant la nouvelle et son mensonge par omission serait pardonné. Qu’il s’offusquât et elle aurait la preuve qu’il se serait opposé à son projet. Elle pensait alors s’engager sur la voie la plus aisée ; désormais qu’elle faisait face au regard agrandi de surprise et de doutes de Sauveur, elle était soudainement moins sûre d’elle.

Elle ne voulait pas le perdre. Elle ne pouvait pas le perdre.

Et, dans le même temps, elle ne désirait rien moins que lui mentir, car c’eût été admettre qu’elle n’était pas dans son bon droit.

[Ce qui est fait est fait,| asséna-t-elle en dardant sur lui des prunelles résolues.|](Sanaa) Il la connaissait assez bien pour comprendre qu’elle lui cachait quelque chose, mais elle ambitionnait encore de détourner son attention du passé pour le pousser à se tourner vers l’avenir. [Je suis enceinte,| répéta-t-elle en affermissant sa voix.| Je suis enceinte et dans moins de trois mois, notre enfant va naître.](Sanaa)

Ne pouvait-il pas se concentrer sur *ça* ?

Dans un mouvement désespéré, elle tendit ses mains vers sa dextre, avec le désir impérieux de poser sa paume sur son ventre inchangé. Depuis qu’elle était certaine de son état et bien qu’elle sût qu’il lui faudrait encore patienter bien des ennéades avant de pouvoir sentir quelque chose, elle ne cessait de se sonder ainsi et, chaque fois, parvenait à se rassénérer un peu plus.
_________________________personnage___________________________
                                           Sanaa Noblegriffon
_____________________________________________________________


Lorsque Sauveur retira vivement sa main et commença à s’éloigner d’elle, Sanaa voulut croire qu’elle avait sa réponse ; elle chercha à se persuader que l’attitude blessée de l’ancien gladiateur lui donnait raison et justifiait ses actes.

Sans succès.

Les mots de Sauveur lui faisaient trop mal ; ils réveillaient chez elle trop de douleurs et trop de craintes. Ils sonnaient trop juste et si faux tout à la fois. Le Hadjaoui pouvait-il feindre découvrir le caractère de sa campagne, quand ils avaient passé tant et tant de temps à s’apprivoiser l’un l’autre pendant plus d’un an ? Avait-il seulement le droit de rejeter si aisément son égoisme, quand elle avait bravé si souvent ses colères noires ? N’était-ce pas l’accord tacite qui les unissait que de s’accepter l’un l’autre, sans concession ? Ne lui devait-il pas une chance de s’expliquer ?

Elle voulait le croire.

Elle en avait *besoin*.

Sauveur, lui, se moquait bien de ses états d’âme et il le lui signifiait de toutes les manières possibles. Il fuyait son contact autant que son regard; il s’ébrouait comme une bête blessée ; il feulait comme un animal en cage. Sanaa, prise de vertige, se découvrait au bord d’un gouffre qu’elle avait creusé avec ses silences et l’épée-louée était désormais hors d’atteinte. Et la Vaanie, ébranlée, de ne pas comprendre pourquoi elle ne répliquait pas à ses attaques. Ses arguments, qu’elle avait ressassés jusqu’à l’écœurement, lui semblaient bien faibles au moment où elle les confrontait à l’ire terrible de l’ancien gladiateur. Pouvait-elle vraiment lui assurer qu’il était libre de se détourner d’elle et de leur enfant, comme elle avait fini par s’en convaincre ? Qu’elle l’éduquerait seule, si c’était là sa volonté ? Elle le *connaissait*, plus qu’aucun Souffle avant lui. Elle *savait* qu’il ne pourrait jamais abandonner la chair de sa chair.

Cette certitude l’avait-elle de quelques manières influencée ? Elle s’en savait capable et sa fierté et son ego la poussaient à accepter et assumer toutes ses fautes, mais dans le même temps, elle comprenait avec une terrible acuité qu’un pas dans cette direction était un pas qui l’éloignait de Sauveur et cette idée lui était insupportable.

Dépassée par la tournure pourtant prévisible des événements, l’adolescente était complètement sans défense quand son compagnon porta le coup de grâce ; défaite, elle le laissa quitter la pièce sans seulement réagir, s’attendant presque à le voir se figer, inspirer profondément, puis rebrousser chemin. Elle se sentit trahie lorsqu’elle comprit qu’il n’en ferait rien.

L’esprit engourdit, elle tituba jusqu’au plat abandonné par le Hadjaoui ; elle darda des prunelles éteintes sur les desserts qu’ils étaient venus chercher et quand ses genoux et ses épaules commencèrent à trembler, elle poussa un cri terrible, le saisit et l’envoya voler au hasard.

Le hasard voulut que Courtois choisit ce moment-là pour la rejoindre. Et la Vaanie de constater, au désespoir, que cet acte de rage ne l’avait nullement soulagée.

____personnage____
Aislinn
_________________

À la mention de Meavh et de Pierre, Sanaa coula un regard discret vers Aislinn ; la Noblegriffon, elle, se passionna pour le fond de sa choppe. <Je n’avais plus entendu leurs noms depuis si longtemps !| songea-t-elle avec une nostalgie poignante.|>(Aislinn) Et l’héritière de se sentir bien seule, subitement. Faisant fi et des convenances et des apparences, la Vaanie posa la paume de sa dextre sur la sénestre de son amie et lui sourit avec douceur. Aislinn recouvrit le dos de sa main avec celle qu’elle avait encore libre, avant de darder des prunelles par trop tristes sur Aimé.

[Mère est partie d’Ys quelques jours après notre rencontre,| expliqua-t-elle avec émotion.| Après m’avoir adoptée…](Aislinn) Les doigts de Sanaa se crispèrent et elle lui lança un regard pour la rassurer. [Tout va bien, il est au courant.](Aislinn)

Aislinn doutait sincèrement que le Vaani eût oublié. Très prudent sur la question de la filiation de sa protégée, Varlar aurait sans doute trouvé à y redire, mais il n’était pas là pour la sermonner et l’adolescente ne se sentait pas en mesure de se méfier d’Aimé ; quand bien même ce dernier l’avait à juste titre mis en garde contre sa proportion manifeste à accorder trop rapidement sa confiance à autrui, la Rivoise ne pouvait tout simplement pas concevoir qu’il pût user de ce savoir contre elle.

[Mère a fait de Varlar, qui nous hébergeait et qu’elle connaissait de longue date, mon tuteur légal,| continua-t-elle finalement après avoir dégagé sa main.| Pierre ne lui a jamais pardonné; ni à moi d’avoir refusé de s’opposer à sa volonté. Nous ne nous voyons plus très souvent.](Aislinn)

Elle chérirait jusqu’à son dernier souffle le Berthildois et elle était persuadée qu’il en allait de même pour lui ; cela ne rendait que plus absurde le fossé qui s’était creusé entre eux. <Il est trop tard pour le combler, désormais. Il est resté à Ys quand moi, je m’installe à Thaar.>(Aislinn) Le pécheur avait refait sa vie dans la petite cité vaanie et Aislinn n’avait pas voulu lui demander d’abandonner tout ce qu’il avait su construire pour elle ; il l’aurait fait sans la moindre hésitation, mais cela n’aurait rien changé à leur relation. <C’est terrible, Ana|, avait-elle un jour dit à sa confidente,| car j’ai l’impression que, quoi que je fasse, je trahis quelqu’un.>(Aislinn)

[Depuis, j’apprends à être à la hauteur de son héritage,| termina-t-elle finalement en esquissant un sourire timide.| Heureusement, Sanaa est là pour m’aider.](Aislinn)

[Elle ne peut plus se passer de moi,| affirma crânement la jeune esclave.|](Sanaa)

Il était clair que la Vaanie n’était pas sérieuse. Bien que sa cadette, elle donnait l’impression de vouloir enlacer la Péninsulaire pour la réconforter et la garder des maux qui l’affligeaient. Cependant, Aislinn était beaucoup trop gentille et se démenait par trop souvent pour faire plaisir à tout le monde, quand bien même cela avait tendance à la mettre dans des états et des situations impossibles; Sanaa intervenait donc rapidement pour la protéger d’elle-même et, en ce sens, ses propos ne manquaient pas de justesse.

Aislinn ne chercha pas à réprimer le sourire que lui inspiraient les paroles de son amie, puis reporta son attention sur Aimé. Elle avait envie de lui poser mille et une questions, mais elle craignait de s’aventurer en terrains dangereux ; il lui avait notamment expliqué deux ans plus tôt que deux de ses frères étaient des esclaves obligés de se battre dans l’arène et elle était terrorisée à l’idée de les mentionner. Ils avaient de grandes chances d’être morts, après tout.

Et dans le même temps, elle ne pouvait pas ne pas envisager de lui proposer son aide pour qu’ils pussent recouvrer leur liberté…
__________________________personnage__________________________
                                              Sanaa Noblegriffon
_____________________________________________________________



Si Sanaa dardait ses prunelles humides sur Courtois, ce n’était pas lui qu’elle regardait ; pas vraiment. En réalité, elle continuait de sonder la porte par laquelle son amant s’en était allé, sans qu’elle fût en mesure de dire si elle s’attendait toujours à le voir revenir sur ses pas. Il n’était donc guère surprenant qu’elle ne prêtât qu’une oreille distraite aux propos de son « beau-frère ». D’ordinaire, elle était capable de faire montre d’une grande patience à son égard, mais les dernières paroles assassines de Sauveur résonnaient encore durement dans son esprit.

[Pas ce soir, Courtois,| finit-elle d’ailleurs par lui répondre après un temps de retard.|](Sanaa)

Et la Vaanie d’accompagner son ordre — sa *supplique* en réalité — d’un hochement de tête désespéré. Elle cligna plusieurs fois des yeux, avant de reporter pleinement son attention sur l’Hadjaoui. Elle avait toujours été capable de passer outre l’hostilité constante qu’il manifestait en tout temps et en tout lieu. Pas cette fois.

[Si tu penses avoir ne serait-ce qu’une once de sagesse à me transmettre, alors par les Cinq, je t’en conjure, attends demain.](Sanaa)

Un rictus douloureux tordit ses lèvres. La question de Courtois était comme un couperet qu’il avait placé au-dessus de sa nuque ; elle ne doutait pas que la réponse qu’il s’apprêtait à lui envoyer en pleine figure n’améliorerait en rien son état, mais c’était parce qu’elle avait une petite idée de ce que l’ancien gladiateur avait en tête. Elle voulait croire, cependant,  que pour mesquine qu’elle trouvait sa manière de faire Courtois essayer de l’*aider* ; elle était simplement persuadée qu’elle n’était pas en état d’en profiter.

[Ce soir, je ne peux pas ; j’ai juste envie de…](Sanaa) Elle ne sut pas comment terminer sa phrase. Elle le désirait, *lui*, mais elle était en train de le perdre. [Rentre chez toi,| lui asséna-t-elle enfin.|](Sanaa)

Elle joignit ses mains dans sa nuque, plaqua ses bras contre ses joues et ferma les yeux en pivotant pour lui tourner le dos.

Elle avait envie d’être seule.

Elle avait besoin de réfléchir.
__________________________personnage__________________________
                                             Sanaa Noblegriffon
_____________________________________________________________

_____________________________date_____________________________
Arcamenel 17 Verimios de l’An `15:XI`, dans la demeure des Hadjaoui.
_____________________________________________________________


Après sa confrontation « malheureuse » avec Sauveur, Sanaa avait tenté tant bien que mal de faire le tri dans ses émotions ; elle avait été aidée en cela par une une Aislinn encore sous le choc d’avoir appris qu’elle serait très bientôt la tante du neveu de Digne. Contrairement à Courtois, qui avait estimé qu’il était important que Sanaa entendît ses vérités, la Rivoise avait pris le parti de ne pas juger les motivations de sa sœur de cœur. Après que la Vaanie se fut un peu calmée — les propos et les actes de l’ancien gladiateur l’avaient mise dans un état à la limite de l’hystérie —, les deux femmes avaient conversé pendant toute la nuit, jusqu’à ce que l’aube du jour suivant les surprît finalement épuisées certes, mais rapprochées surtout. Tout en lui caressant les cheveux dans un geste presque maternel, Aislinn avait assuré la Vaanie de son soutien inconditionnel. Quelques heures plus tard, c’était un Digne sinistre, presque misérable, qui s’invitait dans la demeure des Noblegriffon. De sa bouche, les deux Noblegriffon apprirent le départ inopiné de Sauveur avec Aimé, mais surtout les raisons qui justifiaient ce voyage improvisé.

Sur les conseils d’Aislinn, Sanaa avait pris sur elle pour ne pas débouler chez les Hadjaoui quand, la veille, la nouvelle du retour de Sauveur lui était parvenue, arguant que l’ancien gladiateur avait besoin de repos — ils étaient arrivés à Thaar en fin d’après-midi. Ce faisant, la turbulente Vaanie avait dû lutter contre ses instincts les plus viscéraux ; elle n’avait pas dormi de la nuit. Plantée devant la porte d’entrée de la demeure d’Aimé et de ses frères, elle avait une triste mine, avec ses yeux bouffis et cernés, ses cheveux défaits, son visage sombre et ses épaules voûtées. Digne, qui était resté avec Aislinn cette nuit-là, lui avait assuré quand la Vaanie était finalement partie le retrouver que Sauveur serait seul ; c’était heureux, car Sanaa ne se sentait pas capable d’affronter Aimé — pour qui elle gardait une affection sincère — et moins encore Courtois — dont la simple évocation continuait de la mettre hors d’elle.

La jeune Noblegriffon ne savait pas exactement ce qui la retenait. Elle *voulait* rejoindre Sauveur, mue par cette certitude qu’ils pouvaient s’apporter l’un l’autre la paix qui leur faisait si cruellement défaut ; mais dans le même temps, elle redoutait qu’il la repoussât, quand bien même elle avait tant et tant de choses à lui dire, tant de vérité à lui avouer — ou à lui rappeler, c’était selon — et tant d’amour avec lequel l’enlacer. Elle ne savait plus ce qu’elle était censée faire : elle ne pouvait pas être présente pour Sauveur s’il ne lui pardonnait pas ; elle ne pouvait pas se faire pardonner sans prendre le risque qu’il crût qu’elle se moquait de ce qu’il était en train de traverser ; elle ne pouvait pas non plus se résoudre à rester loin de lui, parce qu’elle était persuadée qu’il avait besoin d’elle.

Quand elle se décida enfin à pénétrer dans la demeure des Hadjaoui, elle n’avait toujours pas trouvé la solution à cette épineuse équation. Heureusement pour elle, Sauveur, qui lui donna l’impression qu’il l’attendait de pied ferme, ne lui laissa guère l’opportunité de s’embourber davantage. Dès qu’il la vit, son visage se ferma et Sanaa crut qu’il allait, ainsi qu’elle l’avait craint, lui demander de s’en aller. À la place, il lui expliqua que s’il ne chercherait jamais à l’empêcher d’agir à sa guise, il ne pourrait plus accepter qu’elle prît des décisions les concernant tous les deux sans lui et voulut qu’elle jurât ne plus recommencer.

Elle était venue à sa rencontre certaine qu’elle devrait le convaincre de lui pardonner ; son ultimatum la désarçonna complètement et elle se jeta à son cou, les larmes aux yeux, en opinant désespérément du chef et en lui répétant plusieurs fois qu’elle regrettait et qu’elle serait toujours là pour lui. Ils s’enlacèrent avec une tendresse rare pendant plusieurs minutes, avant que Sauveur ne cueillît finalement ses lèvres tremblantes.

Avant longtemps, l’épée-louée de la princesse Irohivrah et sa turbulente amante s’offraient l’un à l’autre avec une sincérité et une douceur inédite.

Plus tard, Sanaa fut la première à rompre le silence dans lequel ils s’étaient murés.

[Je t’ai vu mourir, ce jour-là,| lui murmura-t-elle et elle mit tant et tant d’émotions dans ces quelques mots que sa voix se brisa.|](Sanaa) Ils n’avaient jamais évoqué vraiment la défaite terrible de Sauveur fasse à Urgoll’Ven, le Gobelin d’Elda. [Ça me hante encore, certaines nuits. Je revois *son* épée te traverser de part en part, puis tu t’effondres et je ne peux pas te rejoindre.](Sanaa) Elle ne lui avait jamais parlé de ses cauchemars ; en réalité, elle ne s’en était ouverte à personne avant Aislinn, trois jours plus tôt. Elle pleurait, désormais. [Quand je me réveille, je suis persuadée que tu es vraiment mort.](Sanaa)

Sauveur n’avait pas souvent eu l’occasion de voir se confier de la sorte sur ce qu’elle ressentait ; Sanaa taisait beaucoup, tant à Aislinn à qui elle devait tout, qu’à Digne qui demeurait un ami auquel elle tenait, qu’à Aimé qu’elle respectait bien plus que beaucoup. Après ce premier aveu, c’était comme si une digue s’était rompue et très vite, elle se surprit à parler d’autres choses, plus secrètes encore. Elle évoqua cette famille à laquelle on l’avait arrachée quand elle avait sept ans et Sauveur lui racounta quelques souvenirs qu’il gardait de cette mère qu’il était désormais certain d’avoir perdue. Cette nuit-là, ils discutèrent pendant longtemps, bien plus qu’ils ne l’avaient jamais fait.

Et la Vaanie, juste avant de s’endormir, de souffler avec émotion : [Je t’aime, Sauveur…](Sanaa)
_____personnage_____
Aislinn Noblegriffon
___________________

La paume d’Aimé sur son poignet était chaude et douce et Aislinn darda sur lui des prunelles emplies de reconnaissance, car son contact lui faisait du bien. Il la troublait, aussi, réalisa-t-elle avec un temps de retard ; elle baissa vite le regard, embarrassée de sentir ses joues rougir. L’ancien gladiateur interpréta bien mal sa gêne et il s’excusa d’être très «tactile».

[Il n’y a rien à pardonner !| le contredit-elle en relevant la tête.|](Aislinn) <Ça ne m’a pas dérangé… du tout|, garda-t-elle par-devers elle.|>(Aislinn) Elle préféra se tourner vers Sanaa, à la recherche d’un peu de soutien, mais n’en trouva chez son amie aucun. La Vaanie avait l’air d’avoir toutes les peines du monde de retenir un sourire et son regard pétillait d’une malice que la Noblegriffon ne connaissait que trop. Ce fut Aimé qui finalement vint à son secour --- encore une fois --- en reprenant le fil de la conversation comme si de rien était.

[L’est-ce vraiment ?| lui demanda-t-elle.| Mère m’a légué un nom réputé à l’Ouest, qui m’ouvrira bien des portes si jamais je me décide un jour d’y retourner, et une fortune et un commerce qui me protègent ici de bien des maux.](Aislinn)

Elle ne disait pas cela pour vanité, même s’il pouvait être facile de l’interpréter ainsi. Aislinn avait encore beaucoup de difficultés à faire le tri de ses émotions à ce sujet. Elle savait qu’elle aurait mille et mille fois préférait continuer à vivre en toute simplicité à Ys, entourée de Meavh et de Pierre ; elle avait accepté que ce futur-là n’eût jamais appartenu au champ des possibles ; elle comprenait aussi que le legs de sa mère adoptive venait avec un devoir de se montrer à la hauteur de ceux qui l’avaient précédé ; elle ne pouvait cependant s’enlever de la tête qu’elle était une impostrice qui ne méritait pas ce qu’elle possédait.

[Et un tuteur grincheux qui rêve de te dépouiller de ce qui est à toi de plein droit,| s’agaça Sanaa qui pour le coup ne goûtait pas toujours les atermoiements de son amie.|](Sanaa)

[Tu ne devrais pas parler de lui ainsi,| protesta Aislinn avant de darder un regard un peu inquiet vers Aimé et Digne.| Elle exagère, rassurez-vous. Varlar aime l’or, c’est une certitude, mais il sait ce qu’il doit à Mère, tout comme il devine sa colère si jamais il venait à l’aimer *trop*.](Aislinn)

[Tu parles,| pesta Sanaa avant de prendre Digne à témoin.| C’est de la mienne dont il devrait avoir peur.](Sanaa)

Aislinn roula des yeux, mais préféra ne pas mettre de l’huile sur le feu. Elle comprenait l’irritation de Sanaa, mais ce n’était pas un sujet qu’elle était prête à aborder en présence des frères Hadjaoui. <Qu’il me tarde de pouvoir la libérer pour de vrai|, soupira-t-elle plutôt mentalement.|>(Aislinn)

En juge de paix, Aimé réorienta habilement leur discussion et l’adolescente put enfin annoncer sa bonne nouvelle.

[Les affaires de Var…| commença-t-elle avant qu’un raclement de gorge insistant de Sanaa ne l’arrêtât momentanément.| *Nos* affaires se font de plus en plus souvent avec des partenaires qui ne sont pas Yssois ; Varlar a décidé qu’il serait plus profitable que nous vivions à Thaar.](Aislinn) Elle sourit, ravie. [Je suis donc partie pour rester très longtemps, effectivement.](Aislinn)
____personnage_____
Aislinn Noblegriffon
__________________

[Le tenancier... Comment Digne l’a-t-il appelé, déjà ?](Sanaa)

[Kassim... ? Je crois.](Aislinn)

[Eh bien, *Kassim* m’a tout l’air d’être un...](Sanaa)

Aislinn, qui ne connaissait que trop bien son ami, lui donna un discret coup de pied pour l’empêcher d’aller plus loin ; la Vaanie avala son insulte en crachant un petit cri de sus-prise, avant de foudroyer son bourreau de son regard noisette.

[Ose me dire l’inverse,| la défia-t-elle.|](Sanaa)

[Ce n’est pas ça,| souffla Aislinn en baissant d’un ton,| mais s’il t’entend, sur qui crois-tu qu’il va se venger ?](Aislinn)

Sanaa eut un léger mouvement de recul, puis se redressa pour chercher du regard l’employeur des Hadjaoui ; il était parti vaquer un peu plus loin et ne donnait pas l’impression de s’intéresser le moins du monde à ses deux --- uniques --- clientes du moment.

Aislinn comprenait la colère de Sanaa, qui avait longtemps et devait toujours supporter le mépris des hommes et des femmes libres qui se pensaient en droit de la traiter rudement simplement parce qu’elle était une esclave. La Rivoise tendit ses mains et attrapa doucement la paume droite de son amie, puis la massa pour lui signifier son soutien et essayer de l’apaiser ; Sanaa reporta son attention sur sa «maîtresse», poussa un profond soupir pour chasser les dernières bribes d’énervement qui l’agitaient encore.

[Tu l’as retrouvé,| énonça-t-elle finalement avec triomphe.|](Sanaa) D’un mouvement du poignet, elle libéra sa dextre, mais seulement pour mêler ses doigts à ceux d’Aislinn.

[*Nous* l’avons trouvé,| la corrigea la Noblegriffon en esquissant un sourire radieux.| Je n’y serais jamais parvenue sans toi.](Aislinn)

Elles restèrent encore quelques minutes à bavarder, le temps de finir les verres gracieusement offerts par Aimé et son cadet, avant de prendre congé. Juste après l’avoir rabattue derrière elles, Aislinn posa la paume de sa dextre sur la porte de l’auberge et ferma les yeux. Le souvenir des doigts d’Aimé sur la peau de son poignet lui réchauffa les joues et elle murmura : [À très vite, Aimé Hadjaoui…](Aislinn)

_____date___________
Panahos 40 Barkios de l’An `13:XI`, dans l’arrière-cour de XYZ.
___________________

Il n’avait pas fallu longtemps à Aislinn et Sanaa pour faire de XYZ leur seconde maison. Elles s’y sentaient mieux, en vérité, que dans l’impressionnante demeure que Varlar était parvenue à acquérir dans les Soieries, si bien qu’il leur fallait parfois se faire violence pour ne pas leur rendre visite plusieurs fois par ennéades. Elles avaient bien compris que, si Kassam aimait l’or qu’elles amenaient pour payer leurs consommations, il détestait celui qu’il pensait perdre quand elles «distrayaient» Aimé et Digne. Pour leur éviter de s’attirer trop d’ennuis par leurs fautes, elles avaient fini par se résoudre à ne plus venir qu’une fois par ennéade, pendant leur jour de repos ; Aislinn avait proposé à son « héros de l’Agora » de venir même moins régulièrement, pour qu’il pût profiter d’une vraie coupure, mais Aimé lui avait assuré qu’elle ne devait jamais hésiter à franchir le pas de sa porte.

Entre deux visites, elle appliquait le plus rigoureusement le précieux conseil du géant et harcelait littéralement Varlar pour qu’il l’associât aux décisions qu’il prenait au nom de «leur» comptoir Noblegriffon. Il était évident que le Langecin n’en était pas particulièrement ravi de cette nouvelle lubie de sa protégée, mais Aislinn avait pu lui démontrer à plusieurs reprises déjà qu’elle avait la tête bien faite et bourrée d’idées.

Quatre ennéades après ses retrouvailles avec Aimé et sa rencontre avec Digne, le moral de l’héritière Noblegriffon était au beau fixe. Pierre lui manquait toujours et elle s’était promis d’aller lui rendre visite à la fin du mois, mais elle était rassurée de ne plus avoir à affronter ses regards sombres et ses paroles accusatrices.

[Bravo, Digne !| s’enthousiasma Sanaa à côté d’elle en applaudissant les performances du benjamin.| Courage, ton adversaire est vieux et croulant, tu ne vas en faire qu’une bouchée.](Sanaa)

Chassant le Berthildois de ses pensées, Aislinn rapporta son attention sur l’entraînement du jeune Digne par son aîné. Le premier était encore marqué par sa servitude et hanté par celle de ses deux frères, Sauveur et Courtois ; il voulait donc continuer à apprendre à se battre, pour être prêt si un jour l’arène se rappelait à son bon souvenir. C’était quelque chose que le « Briseur de Marteaux » pouvait comprendre.

[Tu passes beaucoup de temps avec Digne, ces derniers temps,| fit remarquer innocemment la Noblegriffon à son amie.|](Aislinn) Elle avait cru déceler la naissance d’une vraie complicité entre Sanaa et le Hadjaoui, mais la première refusait d’en parler quand Aislinn l’interrogeait à ce sujet. Cela piquait un peu plus sa curiosité.

[Tu trouves ?| lui répondit la Vaanie avec amusement.| C’est un homme agréable tant à regarder et qu’avec qui discuter.](Sanaa)

[Je ne saurais pas dire,| objecta Aislinn au moment où une nouvelle passe d’armes s’engageait.| J’ai l’impression qu’il m’évite.](Aislinn)

[Vraiment ? Avec moi, en tout cas, il parle sans cesse de toi.](Sanaa)

Cette confession troubla la Rivoise, qui ne dit rien de plus pendant tout le temps que dura l’entraînement. Quand Aimé décréta la fin des hostilités, les deux adolescentes descendirent du muret sur lequel elles étaient assises et s’approchèrent des deux frères pour les féliciter. Sanaa fut la première à parler et elle surprit tout le monde, à commencer par Aislinn : [Digne, tu tiens encore debout ?](Sanaa) Et la Vaanie de continuer sans attendre de réponse. [Hier soir, Aislinn me disait qu’elle se demandait ce qu’on pouvait ressentir lorsque l’on savait comment se battre.](Sanaa) Elle se tourna vers la principale concernée, qui ne voyait pas du tout où ce début de conversation pouvait bien les emmener. [Digne pourrait t’apprendre, non ? Au moins les bases.](Sanaa)

[Tu n’y penses pas !| protesta la Péninsulaire en ouvrant de grands yeux.| Je suis une femme, je…](Aislinn)

[Pffff !| la coupa Sanaa.| Tu es à Thaar, ici, pas dans l’un de ces châteaux ridicules que vous avez à l’ouest, où vos femelles sont des créatures fragiles et sans défense.](Sanaa) Se tournant vers Digne, elle lui lança un regard appuyé, comme pour l’encourager à prendre le relais.
_____personnage_____
Aislinn Noblegriffon
___________________

Aislinn se sentait et gauche et ridicule et stupide avec son bâton dans les mains ; et dans le même temps, elle ne pouvait s’empêcher de ressentir une pointe d’excitation à tenir ainsi une arme et à l’idée qu’on allait lui apprendre à s’en servir. <Maudites|, songea-t-elle en mirant Sanaa avec un air accusateur.| Toi et mes confidences idiotes.>(Aislinn) La Vaanie lui adressa un sourire d’encouragement pour seule réponse et la Noblegriffon poussa un léger soupir avant de se tourner son instructeur. <C’est vrai qu’il est agréable à regarder|, se sentit-elle obligée de reconnaître tandis qu’il enchaînait quelques étirements.|>(Aislinn) Torse nu, le jeune homme dévoilait un corps sec et musculeux et elle avait beaucoup de peine à s’imaginer dans un accoutrement similaire. <Je suis sûre qu’on me prendrait pour une enfant sous-alimentée|, conclut-elle sombrement.|>(Aislinn) Perdue dans ses pensées, elle ne réagit pas quand il se mit en position et resta bêtement immobile, son bâton devant elle.

Quand il lui expliqua comment elle était censée se placer, elle repoussa toutes ces idées parasites et se concentra complètement sur ses paroles. Ainsi que les Hadjaoui devaient rapidement le découvrir, Aislinn était une forcenée du travail ; lorsqu’elle se lançait dans une tâche, elle ne connaissait pas la demi-mesure. Elle avait par ailleurs une assez bonne intuition et une excellente mémoire, ce qui lui permettait d’apprendre vite. La première instruction de Digne était simple et comme il n’avait pas précisé quel pied elle devait placer devant elle, elle compara son équilibre dans les deux cas. Il lui demanda ensuite de se détendre et de se tenir moins droite, ce qui ne put que lui arracher un discret sourire torve — c’était l’exact opposé des conseils qu’elle avait reçus de Varlar pour se comporter comme une «vraie dame».

Quand elle eut trouvé une pose qui satisfaisait son professeur, elle plia légèrement des genoux plusieurs fois pour mettre ses appuis à l’épreuve. <Cette robe est un cauchemar|, soupira-t-elle par-devers elle.| Assurément, elle a été inventée pour m’empêcher de me battre.>(Aislinn) Elle se promit de choisir un vêtement plus adapté si d’aventure cette expérience était appelée à se perpétuer.

[D’accord,| acquiesça-t-elle avec sérieux quand Digne lui demanda si elle était prête.|](Aislinn) C’était le premier son qu’elle émettait depuis qu’il lui avait tendu son bâton et l’idée que son mutisme concentré lui déstabilisât ne l’avait même pas effleurée.

Les premières «attaques» de l’éphémère esclave n’en étaient pas vraiment, mais elles furent salutaires pour la Noblegriffon qui  put prendre la véritable mesure du poids de son arme et de ce que cela impliquait en termes de motricité. Contrairement à ce qu’avait peut-être craint Digne, elle ne broncha pas aux premiers chocs. Elle était tout simplement trop absorbée par ce qu’elle faisait pour cela; cela écornerait peut-être l’image de jeune fille fragile et sans défense qu’il se faisait d’elle.

[Je ne suis pas sûre de comprendre *comment* je suis censée me servir du bâton,| demanda-t-elle en se reculant d’un pas après trois ou quatre assauts.| Est-ce que je dois être immobile ? Dévier ton arme à toi ? La frapper ?](Aislinn)

C’était là un autre trait de caractère de l’adolescente : quand son instinct ne lui fournissait pas de lui-même les réponses dont elle avait besoin, Aislinn n’hésitait pas à poser *beaucoup* de questions… et à insister tant qu’elle n’était pas pleinement satisfaite.
___________personnage___________
Aislinn Noblegriffon
_______________________________

En élève consciencieuse, Aislinn s’appliqua à assimiler et reproduire les gestes simples de Digne, qui se révélait être un plutôt bon professeur. Il ne fallut en tout cas pas longtemps à la Rivoise pour oublier sa gêne et commencer à prendre un réel plaisir à l’exercice.

[Tu réfléchis vraiment à tout cela, quand tu te bats ?| demanda-t-elle en se laissant par guider par le Vaani.|](Aislinn) Elle buvait ses paroles et ne se formalisa pas le moins du monde de leur proximité ; au contraire, il pouvait ainsi donner corps à ses explications bien plus facilement et cela lui convenait tout à fait. [Face à Aimé, tout à l’heure, vous alliez parfois si vite.](Aislinn)

Évoquer son héros de l’Agora lui rappela qu’il se trouvait encore à quelques mètres d’elle à peine et qu’il avait assisté à toute la scène ; elle rougit légèrement en se demandant si elle avait été ridicule ou non au moment exact où Digne prenait, lui, conscience de l’audace avec laquelle il l’avait approchée. Ils eurent donc tous deux, pour des raisons très différentes, une seconde de flottement avant de se dégager maladroitement. Tandis que son professeur allait reprendre sa place en face d’elle, la Noblegriffon coula un regard vers Aimé et crut lire une note de déception sur son visage qu’elle ne s’expliqua pas ; Sanaa, elle, avait un sourire si large que personne n’aurait été surpris de voir les commissures de ses lèvres frôler ses oreilles. <Que lui a-t-elle dit, encore ?| s’inquiéta-t-elle en resserrant sa prise sur son arme d’entraînement.|>(Aislinn)

[Digne te parle !| lui lança la Vaanie en riant.| Essaie de te reconcentrer ou tu vas finir par te faire mal et tu seras la seule à blâmer.](Sanaa)

L’avertissement de son amie la ramena brutalement les pieds sur terre ; ce qu’elle avait pensé être un discret coup d’œil s’était donc éternisé. Confuse, elle reporta son attention sur Digne, qui donnait l’impression de ne plus savoir où se mettre.

[Je te demande pardon,| souffla-t-elle un peu penaude.| Sanaa n’aime rien plus que me torturer et...](Aislinn) Elle se rendit compte qu’elle s’aventurait sur un terrain glissant --- elle ne se voyait pas dire à Digne que la Vaanie la titillait souvent sur ses sentiments envers son aîné --- et préféra conclure sa phrase avec un sourire d’excuse. [Je veux bien continuer, oui. Je crois que j’ai compris les principes de base.](Aislinn) Elle tira sur sa robe de sa sénestre, avant d’esquisser une grimace amusée et d’ajouter. [Par contre, la prochaine fois, il faudra vraiment que je vienne avec une tenue plus adaptée...](Aislinn)
__________________personnage_____________________
                                       Sanaa
________________________________________________

Digne était d’une humeur bien trop maussade, eu égard aux événements de l’après-midi. Sanaa, d’un naturel toujours optimiste, avait du mal à comprendre sa réaction, mais se garda bien de le brusquer plus encore. En l’occurrence, il avait suffi au Vaani de quelques mots pour lever le voile sur plusieurs sources de ses tourments, chacune menant à sa manière à la même racine profondément ancrée en lui : il manquait beaucoup trop de confiance en lui. La jeune esclave réfléchissait à toute vitesse en se demandant ce qu’elle pouvait dire sans risquer de braquer un peu plus son ami. Elle avait vite admis qu’elle ne pouvait pas lui parler à lui comme elle le faisait avec Aislinn. L’héritière Noblegriffon était d’une patience presque infinie à son égard et lui passait beaucoup trop de choses, quand Digne, lui, avait moins de raison de se montrer aussi compréhensif. C’était d’autant plus dommageable qu’aux yeux de Sanaa, il y avait quelques abcès à crever.

Après quelques secondes d’un silence un peu pesant, elle se rendit compte que — événement peu commun pour elle ! — elle s’était murée dans un mutisme pensif sans prendre la peine de répondre à Digne. Ce n’était pas tant qu’elle n’avait rien à dire — elle avait *retenu* plusieurs répliques en réalité —, mais elle s’était projetée dans les deux ou trois conversations qu’elle considérait devoir avoir avec lui.

[Ça sent très bon, Digne,| lui assura-t-elle avec un aplomb retrouvé en se penchant légèrement en avant au-dessus de la marmite.| Quand j’étais petite, nous avions rarement autant de variété dans nos assiettes.](Sanaa)

Après une demi-seconde d’hésitation, elle coula un regard inquisiteur vers son cuisinier attitré pour jauger de sa réaction. Peu désireuse de le laisser la questionner sur *son* passé, elle décida de lui couper l’herbe sous le pied et enchaîna rapidement sur les mésaventures d’une autre; elle ne doutait pas une seule seconde qu’il mordrait à l’hameçon.

[Tu étais au courant qu’Aislinn avait mendié un an de sa vie dans les rues d’un port de la Péninsule pour pouvoir rejoindre Ys ?| lui demanda-t-elle avec sérieux.|](Sanaa)

Elle n’était pas certaine que la principale concernée approuvât sa confidence ; Sanaa savait qu’elle ne nourrissait aucune honte de cette année éprouvante, mais cela restait des épisodes difficile, marqués par une très grande précarité et chargés de beaucoup d’inquiétudes liées à l’état de santé de sa mère adoptive. L’adolescente espérait cependant que cela ouvrît un peu les yeux à Digne. Concentré sur son amour naissant pour la bourgeoise, il avait apparemment perdu de vue une vérité pourtant essentielle : il ne connaissait pas — encore — Aislinn. Pas vraiment.

Décidant de pousser plus avant la discussion sur ce terrain escarpé, Sanaa insista avec douceur : [Tu crois qu’elle te considérerait différemment si jamais elle venait un jour ici ?](Sanaa) Elle marqua une pause, avant d’ajouter : [Tu as peur que *je* te considère différemment ?](Sanaa)"#;
